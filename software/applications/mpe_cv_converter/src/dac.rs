
//#define __HAL_RCC_DAC_IS_CLK_ENABLED() ((RCC->APB1ENR & (RCC_APB1ENR_DACEN)) != RESET)
//#define __HAL_RCC_DAC_IS_CLK_DISABLED() ((RCC->APB1ENR & (RCC_APB1ENR_DACEN)) == RESET)
//#define __HAL_RCC_DAC_FORCE_RESET() (RCC->APB1RSTR |= (RCC_APB1RSTR_DACRST))
//#define __HAL_RCC_DAC_RELEASE_RESET() (RCC->APB1RSTR &= ~(RCC_APB1RSTR_DACRST))

pub fn init_dac(device: &stm32f7::stm32f7x6::Peripherals, rcc: &stm32f7::stm32f7x6::rcc) {
 
 /* GPIO_InitStruct.Pin = GPIO_PIN_4 | GPIO_PIN_5;
  GPIO_InitStruct.Mode = GPIO_MODE_ANALOG;
  GPIO_InitStruct.Pull = GPIO_NOPULL;
  HAL_GPIO_Init(GPIOA, &GPIO_InitStruct); */

    //init dac gpio pins
    let gpioa = device.GPIOA.split();
    gpioa.pa4.into_floating_input();
    gpioa.pa5.into_floating_input();


  /*##-1- Configure the DAC peripheral #######################################*/
   rcc.regs.apb1enr.modify(|_, w| w.dacen().enabled()); //enable dac

  let dac =  device.DAC
    dac.cd.modify(|_, w| w.en1().enabled());
    dac.cd.modify(|_, w| w. boff1().enabled());
    dac.cd.modify(|_, w| w.en2().enabled());
    dac.cd.modify(|_, w| w. boff2().enabled());

  dac.dhr8r1.modify(|_, w| w.dacc1dhr().set_bit());
  dac.dhr8r2.modify(|_, w| w.dacc2dhr().set_bit());

  /*##-2- Configure DAC channel1 #############################################*/
  /*sConfig.DAC_Trigger = DAC_TRIGGER_NONE;
  sConfig.DAC_OutputBuffer = DAC_OUTPUTBUFFER_DISABLE;

  HAL_DAC_ConfigChannel(&DacHandle, &sConfig, DAC_CHANNEL_1);
  HAL_DAC_ConfigChannel(&DacHandle, &sConfig, DAC_CHANNEL_2);

  HAL_DAC_SetValue(&DacHandle, DAC_CHANNEL_1, DAC_ALIGN_8B_R, 100);
  HAL_DAC_SetValue(&DacHandle, DAC_CHANNEL_2, DAC_ALIGN_8B_R, 100);

  HAL_DAC_Start(&DacHandle, DAC_CHANNEL_1);
  HAL_DAC_Start(&DacHandle, DAC_CHANNEL_2); */


}

/* fn select_clock(&mut self, rcc: &mut Rcc) {
    rcc.regs.apb2enr.modify(|_, w| w.adcen().enabled());
    rcc.regs.cr2.write(|w| w.hsi14on().on());
    while rcc.regs.cr2.read().hsi14rdy().is_not_ready() {}
}

fn power_up(&mut self) {
    if self.rb.isr.read().adrdy().is_ready() {
        self.rb.isr.modify(|_, w| w.adrdy().clear());
    }
    self.rb.cr.modify(|_, w| w.aden().enabled());
    while self.rb.isr.read().adrdy().is_not_ready() {}
} */