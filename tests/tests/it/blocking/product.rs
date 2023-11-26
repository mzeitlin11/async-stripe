use stripe_product::product::{CreateProduct, CreateProductFeatures};

use crate::mock;

#[test]
// FIXME: stripe-mock is missing required `type` field
#[ignore]
// https://github.com/arlyon/async-stripe/issues/437
fn create_product() {
    mock::with_client(|client| {
        let mut create = CreateProduct::new("my product");
        let features = vec![CreateProductFeatures::new("great feature")];
        create.features = Some(&features);

        let product = create.send(client).unwrap();
        assert_eq!(product.features.first().unwrap().name, Some("great feature".into()));
    })
}
