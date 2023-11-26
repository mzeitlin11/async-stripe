use stripe_product::promotion_code::CreatePromotionCode;

use crate::mock;

#[test]
// https://github.com/arlyon/async-stripe/issues/389
fn create_promotion_code() {
    mock::with_client(|client| {
        let mut create = CreatePromotionCode::new("code");
        create.active = Some(true);

        let result = create.send(client).unwrap();
        assert!(result.active);
    })
}
