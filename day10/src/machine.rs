pub struct Machine {
    final_led_state_mask: usize,
    button_wiring_masks: Vec<usize>,
}

impl Machine {
    pub fn new(led_mask: usize, wiring_mask: Vec<usize>) -> Self {
        Self {
            final_led_state_mask: led_mask,
            button_wiring_masks: wiring_mask,
        }
    }
}
