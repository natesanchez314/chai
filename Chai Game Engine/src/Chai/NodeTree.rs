struct NodeTree {
    root: Option<Node>,
    count: int
}

impl NodeTree {
    fn insert(inNode: Node, inParent: Node) {
        if inNode == inParent {
            return;
        }
        if root == null {
            root = inNode;
            inNode.reverse = inNode;
        } else {
            if inParent.forward == null {
                root.reverse = inNode;
            }
            inNode.parent = inParent;
            inNode.prevSibling = null;
            inNode.nextSibling = inParent.child;
            inNode.reverse = inParent;
            inNode.forward = inParent.forward;
            inParent.child.prevSibling = inNode;
            inParent.child = inNode;
            inParent.forward.reverse = inNode;
            inParent.forward = inNode;
        }
        count += 1;
    }

    fn remove(inNode: Node) {
        // Remove branches
        while inNode.child != None {
            remove(inNode.child)
        }
    }
}