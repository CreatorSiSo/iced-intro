let checkbox =
Checkbox::new(self.checkbox_value, "Check me!", Message::CheckboxToggled).style(self.theme);

let slider = Slider::new(
  &mut self.slider,
  0.0..=100.0,
  self.slider_value,
  Message::SliderChanged,
)
.style(self.theme);

let progress_bar = ProgressBar::new(0.0..=100.0, self.slider_value).style(self.theme);