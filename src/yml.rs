use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Node {
    role: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    kind: String,
    #[serde(rename = "apiVersion")]
    api_version: String,
    nodes: Vec<Node>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_cluster() {
        let cluster = Cluster {
            kind: "test".to_owned(),
            api_version: "api/version".to_owned(),
            nodes: vec![
                Node {
                    role: "r1".to_owned(),
                },
                Node {
                    role: "w1".to_owned(),
                },
            ],
        };
        let yaml = serde_yaml::to_string(&cluster).unwrap();
        let want = "\
kind: test
apiVersion: api/version
nodes:
- role: r1
- role: w1
";
        assert_eq!(yaml, want);
    }
}

impl Node {
    pub fn new_control_plane() -> Node {
        Node {
            role: "control-plane".to_owned(),
        }
    }
    pub fn new_worker() -> Node {
        Node {
            role: "worker".to_owned(),
        }
    }
}

const DEFAULT_CLUSTER_KIND: &str = "Cluster";
const DEFAULT_CLUSTER_API_VERSION: &str = "kind.x-k8s.io/v1alpha4";

impl Cluster {
    pub fn new(nodes: Vec<Node>) -> Cluster {
        Cluster {
            kind: DEFAULT_CLUSTER_KIND.to_owned(),
            api_version: DEFAULT_CLUSTER_API_VERSION.to_owned(),
            nodes,
        }
    }
    pub fn generate(control_plane: u8, worker: u8) -> Cluster {
        let c = Node::new_control_plane();
        let w = Node::new_worker();
        let mut nodes = vec![c; control_plane as usize];
        nodes.extend(vec![w; worker as usize]);
        Cluster::new(nodes)
    }
}
