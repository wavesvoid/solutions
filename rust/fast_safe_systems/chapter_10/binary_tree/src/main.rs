use btree::*;


fn main() {
    let mut btree = BinaryTree::with_value(4);

    btree.add(2);
    btree.add(3);
    btree.add(4);
    btree.add(5);
    btree.add(8);
    btree.add(1);

    println!("{:#?}", btree);
}