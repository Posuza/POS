from kivy.uix.boxlayout import BoxLayout
from kivymd.uix.label import MDLabel
from kivy.graphics import Color, RoundedRectangle
from kivy.uix.scrollview import ScrollView
from kivy.metrics import dp
from kivymd.uix.boxlayout import MDBoxLayout
from kivymd.uix.card import MDCard
from kivymd.uix.button import MDIconButton, MDRaisedButton
from kivymd.uix.expansionpanel import MDExpansionPanel, MDExpansionPanelOneLine
from kivymd.uix.behaviors import CommonElevationBehavior
from kivy.core.window import Window

# Custom Divider widget since MDDivider is not available
class Divider(MDBoxLayout):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.size_hint_y = None
        self.height = dp(1)
        self.padding = [0, dp(8)]
        with self.canvas:
            Color(0.5, 0.5, 0.5, 0.5)  # Gray color
            self.rect = RoundedRectangle(pos=self.pos, size=self.size)
        self.bind(pos=self._update_rect, size=self._update_rect)

    def _update_rect(self, instance, value):
        self.rect.pos = instance.pos
        self.rect.size = instance.size

# Add new support-specific divider
class SupportDivider(Divider):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.padding = [0, dp(2)]  # Reduced vertical padding for support items

class HelpContentItem(MDBoxLayout):
    def __init__(self, text="", **kwargs):
        super().__init__(**kwargs)
        self.orientation = 'vertical'
        self.size_hint_y = None
        self.bind(minimum_height=self.setter('height'))  # Make height adaptive
        self.padding = [dp(15), dp(10)]
        
        label = MDLabel(
            text=text,
            theme_text_color="Secondary",
            size_hint_y=None
        )
        label.bind(texture_size=lambda *x: setattr(label, 'height', label.texture_size[1]))
        self.add_widget(label)

class SupportItem(MDBoxLayout):
    def __init__(self, icon, title, description, action_text="", **kwargs):
        super().__init__(**kwargs)
        self.orientation = "horizontal"
        self.padding = [dp(10), dp(2)]  # Reduced vertical padding
        self.spacing = dp(10)
        self.size_hint_y = None
        self.height = dp(80)  # Reduced height for simpler layout
        
        # Left side with icon and text
        left_content = MDBoxLayout(
            orientation="horizontal",
            spacing=dp(10),
            size_hint_x=0.7,
            pos_hint={'center_y': 0.5}  # Center vertically
        )
        
        icon_button = MDIconButton(
            icon=icon,
            theme_text_color="Custom",
            text_color=[0.2, 0.4, 0.8, 1],
            pos_hint={'center_y': 0.5}  # Center vertically
        )
        
        text_content = MDBoxLayout(
            orientation="vertical",
            spacing=dp(2),
            padding=[0, dp(5)],
            pos_hint={'center_y': 0.5}  # Center vertically
        )
        
        title_label = MDLabel(
            text=title,
            theme_text_color="Primary",
            font_style="Subtitle1",
            size_hint_y=None,
            height=dp(30)
        )
        
        desc_label = MDLabel(
            text=description,
            theme_text_color="Secondary",
            font_style="Caption",
            size_hint_y=None,
            height=dp(20)
        )
        
        text_content.add_widget(title_label)
        text_content.add_widget(desc_label)
        
        left_content.add_widget(icon_button)
        left_content.add_widget(text_content)
        
        self.add_widget(left_content)
        
        # Right side with action button
        if action_text:
            self.action_button = MDRaisedButton(
                text=action_text,
                size_hint_x=0.3,
                md_bg_color=[0.2, 0.4, 0.8, 1],
                pos_hint={'center_y': 0.5}  # Center vertically
            )
            self.add_widget(self.action_button)

class HelpView(MDBoxLayout):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.orientation = "vertical"
        self.spacing = dp(0)  # Reduced spacing
        self.padding = [dp(20), dp(20)]  # Remove vertical padding
        
        # Bind window size changes
        Window.bind(size=self._update_layout)
        
        # Add top divider
        self.add_widget(Divider())
        
        # Quick Help Section fixed at top
        self.quick_help = SupportItem(
            icon="lightbulb",
            title="Quick Start Guide",
            description="Learn the basics of using the POS system",
            action_text="VIEW GUIDE"
        )
        self.add_widget(self.quick_help)
        
        # Add divider after quick help
        self.add_widget(Divider())
        
        # Create scrollable container
        self.scroll = ScrollView()
        self.container = MDBoxLayout(
            orientation="vertical",
            spacing=dp(15),  # Reduced spacing
            size_hint_y=None,
            padding=[dp(10), dp(5)]  # Reduced vertical padding
        )
        self.container.bind(minimum_height=self.container.setter('height'))
        
        # FAQ Section with responsive layout
        self.faq_container = MDBoxLayout(
            orientation="vertical",
            spacing=dp(10),
            size_hint_y=None
        )
        self.faq_container.bind(minimum_height=self.faq_container.setter('height'))
        
        faq_items = [
            ("How do I process a sale?", 
             "1. Click on Sales\n2. Scan items or add manually\n3. Select payment method\n4. Complete transaction"),
            ("How to issue a refund?",
             "1. Go to Sales History\n2. Find the transaction\n3. Click Refund\n4. Follow the prompts"),
            ("How to add new products?",
             "1. Go to Products\n2. Click Add New\n3. Fill in details\n4. Save changes"),
            ("How to close the day?",
             "1. Go to Reports\n2. Select Daily Summary\n3. Click Close Day\n4. Print report")
        ]
        
        for question, answer in faq_items:
            self.faq_container.add_widget(MDExpansionPanel(
                icon="help-circle",
                content=HelpContentItem(text=answer),
                panel_cls=MDExpansionPanelOneLine(
                    text=question
                )
            ))
        
        self.container.add_widget(self.faq_container)        
        # Support Options with Simple Vertical Layout
        self.support_container = MDBoxLayout(
            orientation="vertical",
            spacing=dp(0),  # Remove spacing since we'll use dividers
            size_hint_y=None
        )
        self.support_container.bind(minimum_height=self.support_container.setter('height'))

        # Support items in vertical layout
        support_items = [
            ("phone", "Phone Support", "Call our 24/7 support line", "CALL NOW"),
            ("email", "Email Support", "Send us your questions", "EMAIL"),
            ("chat", "Live Chat", "Chat with our support team", "START CHAT"),
            ("web", "Knowledge Base", "Browse our help articles", "BROWSE")
        ]

        # Add items with dividers inside support_container
        for i, item in enumerate(support_items):
            if i > 0:  # Add divider before each item except the first one
                self.support_container.add_widget(SupportDivider())
            self.support_container.add_widget(SupportItem(*item))

        self.container.add_widget(self.support_container)
        
        # Add divider before version info
        self.container.add_widget(Divider())
        
        # Add version info
        self.version_box = MDBoxLayout(
            orientation="vertical",
            size_hint_y=None,
            height=dp(40),  # Reduced height
            padding=[0, dp(10)]  # Reduced padding
        )
        
        self.version_box.add_widget(MDLabel(
            text="POS System v1.0.0",
            theme_text_color="Secondary",
            halign="center"
        ))
        
        self.container.add_widget(self.version_box)
        self.scroll.add_widget(self.container)
        self.add_widget(self.scroll)
        
        # Remove bottom divider (deleted the following line)
        # self.add_widget(Divider())

    def _update_layout(self, instance, size):
        """Update layout based on window size"""
        width = size[0]
        
        # Update container padding and spacing only
        self.container.padding = [
            dp(10) if width < dp(600) else dp(20),
            dp(5)  # Reduced vertical padding
        ]
        self.container.spacing = dp(15) if width >= dp(600) else dp(10)  # Reduced spacing

