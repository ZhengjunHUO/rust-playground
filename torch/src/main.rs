use tch::{Device, IndexOp, Kind, Tensor, nn, nn::Module, nn::OptimizerConfig};

/// A simple expert: 2-layer MLP (Linear -> ReLU -> Linear).
fn expert(vs: &nn::Path, input_dim: i64, hidden_dim: i64, output_dim: i64) -> nn::Sequential {
    let mut seq = nn::seq();
    seq = seq.add(nn::linear(vs, input_dim, hidden_dim, Default::default()));
    seq = seq.add_fn(|xs| xs.relu());
    seq = seq.add(nn::linear(vs, hidden_dim, output_dim, Default::default()));
    seq
}

/// Gating network: Linear -> Softmax over experts.
fn gating(vs: &nn::Path, input_dim: i64, num_experts: i64) -> nn::Sequential {
    let mut seq = nn::seq();
    seq = seq.add(nn::linear(vs, input_dim, num_experts, Default::default()));
    // Softmax along last dim
    seq = seq.add_fn(|xs| xs.softmax(-1, Kind::Float));
    seq
}

struct MoE {
    experts: Vec<nn::Sequential>,
    gate: nn::Sequential,
}

impl MoE {
    fn new(
        vs: &nn::Path,
        input_dim: i64,
        hidden_dim: i64,
        output_dim: i64,
        num_experts: i64,
    ) -> Self {
        let mut experts = Vec::new();
        for i in 0..num_experts {
            experts.push(expert(
                &vs.sub(&format!("expert_{i}")),
                input_dim,
                hidden_dim,
                output_dim,
            ));
        }
        let gate = gating(&vs.sub("gate"), input_dim, num_experts);
        Self { experts, gate }
    }

    /// Forward: weighted sum of expert outputs, weights given by gate.
    fn forward(&self, xs: &Tensor) -> Tensor {
        // weights: [batch, num_experts]
        let weights = self.gate.forward(xs);

        // For each expert i:
        //   yi = expert_i(xs)           [batch, output_dim]
        //   wi = weights[:, i].unsq(-1) [batch, 1]
        //   contrib_i = yi * wi         [batch, output_dim]
        let mut contribs: Vec<Tensor> = Vec::new();
        for (i, expert) in self.experts.iter().enumerate() {
            let yi = expert.forward(xs);
            let wi = weights.i((.., i as i64)).unsqueeze(-1);
            contribs.push(yi * wi);
        }

        // Sum across experts (new dim 0 is expert axis)
        Tensor::stack(&contribs, 0).sum_dim_intlist(&[0 as i64][..], false, Kind::Float)
    }

    /// Convenience: return gate weights for inputs (to inspect routing).
    fn gate_weights(&self, xs: &Tensor) -> Tensor {
        self.gate.forward(xs) // [batch, num_experts]
    }
}

fn main() {
    let device = Device::Cpu;
    tch::manual_seed(88);

    let vs = nn::VarStore::new(device);
    let root = &vs.root();

    let input_dim = 2;
    let hidden_dim = 16;
    let output_dim = 1;
    let num_experts = 3;
    let moe = MoE::new(root, input_dim, hidden_dim, output_dim, num_experts);

    let mut opt = nn::Adam::default().build(&vs, 1e-2).unwrap();

    // Preapare dataset for training
    // Cluster A (~(1,1)) -> label 1
    // Cluster B (~(-1,-1)) -> label 0
    let inputs = Tensor::from_slice2(&[
        [1.0, 1.0],
        [1.2, 0.9],
        [0.8, 1.1],
        [1.1, 1.2],
        [0.9, 0.8],
        [1.3, 1.0],
        [-1.0, -1.0],
        [-1.1, -0.9],
        [-0.8, -1.2],
        [-1.2, -1.1],
        [-0.9, -0.8],
        [-1.3, -1.0],
    ])
    .to_device(device) // CPUDoubleType{12,2}
    .to_kind(Kind::Float); // CPUFloatType{12,2} 
    // inputs.print();

    let labels = Tensor::from_slice(&[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0])
        .unsqueeze(-1)
        .to_device(device)
        .to_kind(Kind::Float);

    // Training
    println!("### Training ...");
    let epochs = 300;
    for epoch in 1..=epochs {
        let preds = moe.forward(&inputs); // [batch, 1]
        let loss = (preds - &labels).pow_tensor_scalar(2.0).mean(Kind::Float); // MSE

        opt.backward_step(&loss);

        if epoch % 50 == 0 {
            //println!("Epoch {epoch:>3}");
            println!("    Epoch {epoch:>3}: loss = {:.6}", loss.double_value(&[]));
        }
    }

    // Inference
    println!("\n### Inference:");
    let test = Tensor::from_slice2(&[
        [1.0, 1.0],
        [-1.0, -1.0],
        [0.0, 0.0],
        [1.3, 1.1],
        [-1.2, -0.9],
    ])
    .to_device(device)
    .to_kind(Kind::Float);

    let w = moe.gate_weights(&test); // [5, num_experts]
    println!("=> Gate weights (rows = inputs, cols = experts): {:?}", w);
    w.print();

    // Which expert did the gate favor? (argmax over experts)
    let (_vals, idx) = w.max_dim(1, false);
    println!(
        "=> Argmax expert per input, expect [a, b, a, a, b]: {:?}",
        idx
    );
}
