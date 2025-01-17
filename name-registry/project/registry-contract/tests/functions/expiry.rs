mod success {
    use crate::utils::{
        abi::{expiry, extend, register},
        setup, EXTEND_DURATION, REGISTER_DURATION,
    };

    #[tokio::test]
    async fn can_get_expiry() {
        let (instance, acc, _wallet2) = setup().await;

        register(
            &instance,
            &acc.name,
            REGISTER_DURATION,
            &acc.identity(),
            &acc.identity(),
        )
        .await;
        let previous_expiry_response = expiry(&instance, &acc.name).await;

        extend(&instance, &acc.name, EXTEND_DURATION).await;

        let new_expiry_response = expiry(&instance, &acc.name).await;

        assert_eq!(
            previous_expiry_response.0.value.unwrap() + EXTEND_DURATION,
            new_expiry_response.0.value.unwrap()
        );
    }
}

mod revert {
    use crate::utils::{abi::expiry, setup};

    #[tokio::test]
    #[should_panic(expected = "`Result::unwrap()` on an `Err` value")]
    async fn cant_get_expiry() {
        let (instance, acc, _wallet2) = setup().await;

        let expiry = expiry(&instance, &acc.name).await;
        expiry.0.value.unwrap();
    }
}
