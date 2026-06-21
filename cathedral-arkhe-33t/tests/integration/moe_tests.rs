use cathedral_arkhe_33t::config::MoEConfig;
use cathedral_arkhe_33t::moe::MoELayer;
use cathedral_arkhe_33t::tensor::Tensor;

#[test]
fn test_moe_creation() {
    let config = MoEConfig {
        num_experts: 16,
        top_k: 4,
        hidden_size: 8,
        intermediate_size: 32,
        capacity_factor: 1.25,
        load_balancing_loss_coef: 0.01,
    };
    let moe = MoELayer::new(&config);
    assert_eq!(moe.num_experts, 16);
    assert_eq!(moe.top_k, 4);
}

#[test]
fn test_moe_forward() {
    let config = MoEConfig {
        num_experts: 4,
        top_k: 2,
        hidden_size: 8,
        intermediate_size: 32,
        capacity_factor: 1.25,
        load_balancing_loss_coef: 0.01,
    };
    let mut moe = MoELayer::new(&config);
    let x = Tensor::randn((2, 8));
    let (output, aux_loss) = moe.forward(&x);

    assert_eq!(output.shape(), (2, 8));
    assert!(aux_loss >= 0.0);
}
