// Iterating Through Options
fn main() {
    let some_product = Some("laptop");
    let mut products = vec!["cellphone", "battery", "charger"];

    match some_product {
        Some(product) => products.push(product),
        _ => {}
    }

    if let Some(product) = some_product {
        products.push(product);
    }

    // collect two collections' two iterator into one
    let prod_iter = products.iter().chain(some_product.iter());

    for item in prod_iter {
        println!("item value is {:?}", item);
    }

    // here is the demo for Options
    let opt_prods =
        vec![Some("a"), Some("b"), Some("c"), None, None, None, Some("d")];
    let mut prod_without_none = Vec::new();

    for p in opt_prods.clone() {
        if p.is_some() {
            prod_without_none.push(p.unwrap());
        }
    }

    // direclty use the filter to filter NOne in the vectors
    let mut ret = opt_prods
        .clone()
        .into_iter()
        .filter(|item| item.is_some())
        .map(|item| item.unwrap())
        .collect::<Vec<&str>>();

    println!("ret value content : {:?}", ret);

    let items = vec![("a", "b"), ("c", ""), ("d", "e")];

    // we can use the flatten to split all the tuples items in item s
    let mut ret = items
        .into_iter()
        .flat_map(|item| {
            let mut rett = Vec::new();
            rett.push(item.0);
            rett.push(item.1);
            rett
        })
        .filter(|str| str.len() != 0)
        .collect::<Vec<&str>>();

    println!("ret value {:?}", ret);
}
