from kivy.uix.boxlayout import BoxLayout
from kivy.graphics import Color, RoundedRectangle
from kivymd.uix.boxlayout import MDBoxLayout
from kivymd.uix.label import MDLabel
from kivymd.uix.button import MDRaisedButton
from kivy.metrics import dp

class DefaultView(MDBoxLayout):
    def __init__(self, name, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.padding = [15, 15, 15, 15]
        # self.size_hint = (1, None)
        
        with self.canvas.before:
            Color(0.2, 0.3, 0.4, 1)
            self.bg = RoundedRectangle(pos=self.pos, size=self.size, radius=[15])
        self.bind(pos=self._update_rect, size=self._update_rect)

        # Create a centered box for content with bottom margin
        content_box = MDBoxLayout(
            orientation='vertical', 
            spacing=dp(10),
            size_hint_x=None,
            width=dp(250),
            pos_hint={'center_x': 0.5},
            padding=[0, 0, 0, dp(280)]  # Added bottom margin of 50dp
        )

        label = MDLabel(
            text=self.replace_underscores(f'{name} have no Data'),
            halign="center",
            theme_text_color="Custom",
            text_color=(1, 1, 1, 1),
            height=dp(40),
            size_hint_y=None
        )
        content_box.add_widget(label)

        button = MDRaisedButton(
            text="Add",
            height=dp(40),
            size_hint_x=None,
            width=dp(100),
            pos_hint={'center_x': 0.5}
        )
        content_box.add_widget(button)

        self.add_widget(content_box)

    def replace_underscores(self, text):
        return text.replace('_', ' ')

    def _update_rect(self, instance, value):
        self.bg.pos = instance.pos
        self.bg.size = instance.size