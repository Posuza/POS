from kivy.uix.modalview import ModalView
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.gridlayout import GridLayout
from kivy.uix.button import Button
from kivy.uix.label import Label
from kivy.metrics import dp
from kivy.graphics import Color, RoundedRectangle

class CheckoutPopup(ModalView):
    def __init__(self, total_amount, on_confirm, **kwargs):
        super().__init__(**kwargs)
        self.size_hint = (0.8, 0.8)  # Made taller for calculator
        self.background_color = [0, 0, 0, 0.5]
        self.total_amount = total_amount
        
        # Main container
        container = BoxLayout(
            orientation='vertical',
            padding=dp(20),
            spacing=dp(15)
        )
        
        # White background with rounded corners
        with container.canvas.before:
            Color(0.95, 0.95, 0.95, 1)  # Light gray
            self.rect = RoundedRectangle(
                pos=container.pos,
                size=container.size,
                radius=[20,]
            )
        container.bind(pos=self._update_rect, size=self._update_rect)
        
        # Title section
        header = BoxLayout(
            orientation='vertical',
            size_hint_y=0.1
        )
        header.add_widget(Label(
            text='Confirm Checkout',
            font_size='30sp',
            bold=True,
            color=(0.2, 0.2, 0.2, 1)
        ))
        container.add_widget(header)

        # Replace the table_grid section with this new implementation
        table_grid = GridLayout(
            cols=3,
            spacing=dp(10),
            size_hint_y=0.2,
            padding=[dp(10), dp(5)]
        )

        # Create labels for the top row
        labels = [
            Label(
                text='Payment',
                font_size='18sp',
                color=(0.2, 0.2, 0.2, 1),
                bold=True,
                size_hint_y=None,
                height=dp(30)
            ),
            Label(
                text='Total Amount',
                font_size='18sp',
                color=(0.2, 0.2, 0.2, 1),
                bold=True,
                size_hint_y=None,
                height=dp(30)
            ),
            Label(
                text='Change',
                font_size='18sp',
                color=(0.2, 0.2, 0.2, 1),
                bold=True,
                size_hint_y=None,
                height=dp(30)
            )
        ]

        # Add the top row labels
        for label in labels:
            table_grid.add_widget(label)

        # Create and add the value labels
        self.payment_label = Label(
            text='$0.00',
            font_size='24sp',
            color=(0.2, 0.2, 0.8, 1),  # Changed to blue
            bold=True
        )

        self.total_label = Label(
            text=f'${total_amount:.2f}',
            font_size='24sp',
            color=(0.2, 0.2, 0.2, 1),
            bold=True
        )
        
        self.change_label = Label(
            text='',  # Changed to empty string initially
            font_size='24sp',
            color=(0.2, 0.8, 0.2, 1),  # Changed to green
            bold=True
        )

        table_grid.add_widget(self.payment_label)
        table_grid.add_widget(self.total_label)
        table_grid.add_widget(self.change_label)
        
        container.add_widget(table_grid)

        # Calculator buttons (adjust size_hint_y since we removed payment display)
        calc_grid = GridLayout(
            cols=3,
            spacing=dp(10),
            size_hint_y=0.7  # Increased from 0.5 to fill space
        )

        # Define all buttons in order
        buttons_config = [
            ('Clr', (0.9, 0.9, 0.9, 1), (0.8, 0.2, 0.2, 1)),  # Changed from 'C' to 'CLEAR'
            ('1', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('2', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('3', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('4', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('5', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('6', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('7', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('8', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('9', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('0', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1)),
            ('.', (0.9, 0.9, 0.9, 1), (0.2, 0.2, 0.2, 1))
        ]

        # Create and add buttons
        for text, bg_color, text_color in buttons_config:
            btn = Button(
                text=text,
                font_size='20sp' if text == 'CLEAR' else '24sp',  # Smaller font for CLEAR
                background_normal='',
                background_color=bg_color,
                color=text_color,
                bold=True
            )
            btn.bind(on_press=self._on_button_press)
            calc_grid.add_widget(btn)

        container.add_widget(calc_grid)
        
        # Action buttons
        buttons = BoxLayout(
            orientation='horizontal',
            size_hint_y=0.15,
            spacing=dp(20)
        )
        
        # Cancel button
        cancel_btn = Button(
            text='Cancel',
            size_hint_x=0.5,
            background_color=(0.8, 0.2, 0.2, 1),
            background_normal='',
            font_size='18sp',
            bold=True
        )
        cancel_btn.bind(on_press=self.dismiss)
        
        # Confirm button
        self.confirm_btn = Button(
            text='Pay',
            size_hint_x=0.5,
            background_color=(0.2, 0.8, 0.2, 1),
            background_normal='',
            font_size='18sp',
            bold=True
        )
        self.confirm_btn.bind(on_press=lambda x: self._on_confirm(on_confirm))
        
        buttons.add_widget(cancel_btn)
        buttons.add_widget(self.confirm_btn)
        container.add_widget(buttons)
        
        self.add_widget(container)
        
        # Initialize payment amount
        self.current_amount = '0'
        self._update_display()

    def _on_button_press(self, instance):
        if instance.text == 'Clr':  # Changed from 'CLEAR' to 'Clr'
            self.current_amount = '0'
            self.payment_label.text = '$0.00'
            self.change_label.text = ''  # Clear change label
            self.confirm_btn.disabled = True
            return
        elif instance.text == '.' and '.' in self.current_amount:
            return
        elif instance.text == '0' and self.current_amount == '0':
            return
        elif instance.text == '.' and self.current_amount == '0':
            self.current_amount = '0.'
        elif self.current_amount == '0':
            self.current_amount = instance.text
        else:
            self.current_amount += instance.text
        self._update_display()

    def _update_display(self):
        try:
            amount = float(self.current_amount)
            self.payment_label.text = f'${amount:.2f}'
            
            # Calculate and display change
            if amount > 0:
                change = amount - self.total_amount
                self.change_label.text = f'${max(0, change):.2f}' if amount >= self.total_amount else ''
            else:
                self.change_label.text = ''
            
            # Enable/disable confirm button based on payment amount
            self.confirm_btn.disabled = amount < self.total_amount
        except ValueError:
            self.payment_label.text = 'Error'
            self.change_label.text = ''
            self.confirm_btn.disabled = True

    def _update_rect(self, instance, value):
        self.rect.pos = instance.pos
        self.rect.size = instance.size
        
    def _on_confirm(self, callback):
        callback()  # This will now call clear_cart from CartSection
        self.dismiss()
