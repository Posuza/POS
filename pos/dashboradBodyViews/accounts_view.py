
from kivy.uix.boxlayout import BoxLayout
from kivymd.uix.label import MDLabel
from kivy.graphics import Color, RoundedRectangle
from kivy.uix.scrollview import ScrollView
from kivy.metrics import dp
from kivymd.uix.tab import MDTabsBase
from kivymd.uix.floatlayout import MDFloatLayout
from kivymd.uix.boxlayout import MDBoxLayout
from kivymd.uix.card import MDCard
from kivymd.uix.button import MDIconButton
from kivymd.uix.list import MDList, TwoLineIconListItem, IconLeftWidget
from kivymd.uix.selectioncontrol import MDSwitch
from kivymd.uix.tab import MDTabs

class Tab(MDFloatLayout, MDTabsBase):
    def __init__(self, tab_name, **kwargs):
        super().__init__(**kwargs)
        self.tab_label_text = tab_name

class SettingCard(MDCard):
    def __init__(self, title, icon, description, has_switch=True, **kwargs):
        super().__init__(**kwargs)
        self.orientation = "horizontal"
        self.padding = dp(15)
        self.spacing = dp(15)
        self.size_hint_y = None
        self.height = dp(80)
        self.md_bg_color = [0.9, 0.9, 0.9, 1]
        self.radius = [10]

        self.icon = MDIconButton(
            icon=icon,
            pos_hint={"center_y": 0.5},
            theme_text_color="Custom",
            text_color=[0.3, 0.3, 0.3, 1]
        )

        text_box = MDBoxLayout(
            orientation="vertical",
            spacing=dp(4),
            size_hint_x=0.7
        )

        title_label = MDLabel(
            text=title,
            font_style="Subtitle1",
            theme_text_color="Custom",
            text_color=[0.1, 0.1, 0.1, 1]
        )

        desc_label = MDLabel(
            text=description,
            theme_text_color="Custom",
            text_color=[0.5, 0.5, 0.5, 1],
            font_style="Caption"
        )

        text_box.add_widget(title_label)
        text_box.add_widget(desc_label)

        if has_switch:
            self.switch = MDSwitch(
                pos_hint={"center_y": 0.5},
                size_hint=(None, None),
                size=(dp(45), dp(25))
            )
        else:
            self.switch = MDIconButton(
                icon="chevron-right",
                pos_hint={"center_y": 0.5},
                theme_text_color="Custom",
                text_color=[0.5, 0.5, 0.5, 1]
            )

        self.add_widget(self.icon)
        self.add_widget(text_box)
        self.add_widget(self.switch)

class StaffAccountsView(MDBoxLayout):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.orientation = "vertical"
        self.spacing = dp(10)
        self.padding = dp(10)

        self.tabs = MDTabs(
            background_color=[0.9, 0.9, 0.9, 1],
            text_color_normal=[0.5, 0.5, 0.5, 1],
            text_color_active=[0.2, 0.4, 0.8, 1],
            indicator_color=[0.2, 0.4, 0.8, 1]
        )

        # Single tab_data configuration
        tab_data = {
            "General": [
                ("wifi", "Network Settings", "Configure network connection"),
                ("cloud", "Auto Backup", "Backup data automatically"),
                ("update", "System Updates", "Check for system updates"),
                ("security", "Security", "Configure security settings")
            ],
            "Interface": [
                ("weather-night", "Dark Mode", "Enable dark theme"),
                ("format-size", "Text Size", "Adjust display text size"),
                ("palette", "Colors", "Customize theme colors"),
                ("monitor", "Display", "Configure display settings")
            ],
            "Payment": [
                ("cash", "Cash Payment", "Enable cash payments"),
                ("credit-card", "Card Payment", "Configure card payments"),
                ("bluetooth", "Contactless", "Enable contactless payments"),
                ("qrcode", "QR Codes", "Configure QR code payments")
            ],
            "Printing": [
                ("printer", "Receipt Printer", "Configure receipt printer"),
                ("file-document-outline", "Paper Size", "Set receipt paper size"),
                ("content-copy", "Auto Print", "Print receipts automatically"),
                ("printer-settings", "Advanced", "Advanced printer settings")
            ]
        }

        for tab_name, settings in tab_data.items():
            tab = Tab(tab_name=tab_name)
            scroll = ScrollView()
            content = MDBoxLayout(
                orientation="vertical",
                spacing=dp(10),
                padding=dp(10),
                size_hint_y=None
            )
            content.bind(minimum_height=content.setter('height'))

            for icon, title, desc in settings:
                content.add_widget(
                    SettingCard(
                        title=title,
                        icon=icon,
                        description=desc,
                        has_switch=True if "Enable" in desc else False
                    )
                )

            scroll.add_widget(content)
            tab.add_widget(scroll)
            self.tabs.add_widget(tab)

        self.add_widget(self.tabs)

