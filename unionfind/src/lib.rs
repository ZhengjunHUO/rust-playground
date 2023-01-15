pub struct UF {
    count: usize,
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UF {
    pub fn new(num: usize) -> UF {
        let mut parent = Vec::with_capacity(num);
        for i in 0..num {
            parent.push(i);
        }

        UF {
            count: num,
            parent,
            size: vec![1; num],
        }
    }

    pub fn find_root(&mut self, n: usize) -> usize {
        let mut rslt = n;
        while rslt != self.parent[rslt] {
            if self.parent[rslt] != self.parent[self.parent[rslt]] {
                self.size[self.parent[rslt]] -= self.size[rslt];
                self.parent[rslt] = self.parent[self.parent[rslt]];
            }
            rslt = self.parent[rslt];
        }

        rslt
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let ar = self.find_root(a);
        let br = self.find_root(b);
        if ar == br {
            return
        }

        if self.size[ar] < self.size[br] {
            self.parent[ar] = br;
            self.size[br] += self.size[ar];
        } else {
            self.parent[br] = ar;
            self.size[ar] += self.size[br];
        }

        self.count -= 1;
    }

    pub fn is_linked(&mut self, a: usize, b: usize) -> bool {
        self.find_root(a) == self.find_root(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UF::new(10);
        uf.union(0, 1);
        uf.union(6, 0);
        uf.union(2, 3);
        uf.union(2, 5);
        uf.union(1, 3);
        uf.find_root(3);

        assert_eq!(uf.count, 5);
        assert_eq!(uf.parent, vec![0,0,0,0,4,2,0,7,8,9]);
        assert_eq!(uf.size, vec![6,1,2,1,1,1,1,1,1,1]);
        assert!(uf.is_linked(5,6));
    }
}

/*
	uf.Union(6,5)
	if uf.IsLinked(3,5) != true {
		t.Errorf("Node 3 and 5 should be linked!\n")
	}

	// restore the snapshot
	uf.SetParent(p)
	uf.SetSize(s)
	uf.SetCount(c)
	if uf.Count() != c || !reflect.DeepEqual(uf.Parent(), p) || !reflect.DeepEqual(uf.Size(), s) {
		t.Errorf("Union find returns count: %v, parent: %v, size: %v\nexpect count: %v, parent: %v, size: %v\n",
			uf.Count(), uf.Parent(), uf.Size(), c, p, s)
	}
*/
