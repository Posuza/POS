from kivy.uix.boxlayout import BoxLayout
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.metrics import dp
from kivy.core.window import Window

class CartItem(BoxLayout):
    def __init__(self, product, price, quantity, delete_callback, **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'horizontal'
        self.size_hint_y = None
        self.height = dp(40)
        self.spacing = dp(10)
        self.padding = [0, 0, 0, 0]

        # Product name with text truncation for tablet
        display_text = self._format_product_text(product)
        product_label = Label(
            text=display_text,
            size_hint_x=0.4,
            halign='left',
            valign='middle',
            text_size=(None, dp(30)),
            # padding=[dp(1), 0],
            color=(0.4, 0.4, 0.4, 1)
        )
        product_label.bind(size=lambda *_: setattr(product_label, 'text_size', (product_label.width, dp(40))))
        self.add_widget(product_label)
        
        # Quantity in red
        self.add_widget(Label(
            text=f"x{quantity}",
            size_hint_x=0.2,
            color=(0.8, 0.2, 0.2, 1),
            font_size='14sp',
            bold=True
        ))
        
        # Combined price column with stacked prices
        price_container = BoxLayout(
            orientation='vertical',
            size_hint_x=0.3,
            spacing=dp(1)
        )
        
        # Unit price on top
        price_container.add_widget(Label(
            text=f"${price:.2f}",
            size_hint_y=0.1,
            font_size='12sp',
            color=(0.4, 0.4, 0.4, 1)
        ))
        
        # Total price below
        price_container.add_widget(Label(
            text=f"${price * quantity:.2f}",
            size_hint_y=0.2,
            font_size='13sp',
            bold=True,
            color=(0.4, 0.4, 0.4, 1)
        ))
        
        self.add_widget(price_container)
        
        # Delete button with margins
        delete_container = BoxLayout(
            orientation='vertical',
            size_hint_x=0.15,
            padding=[0, dp(5), 0, dp(5)]  # [left, top, right, bottom]
        )
        
        delete_btn = Button(
            text="×",
            size_hint=(1, 0.5),  # Fill container
            background_color=(0.8, 0.2, 0.2, 1),
            background_normal='',
            font_size='20sp',
            bold=True
        )
        delete_btn.bind(on_press=lambda x: delete_callback(product, price))
        delete_container.add_widget(delete_btn)
        self.add_widget(delete_container)

    def _format_product_text(self, text):
        if Window.width <= 1024:
            words = text.split()
            if len(words) > 7:
                return ' '.join(words[:7]) + '...'
        return text
