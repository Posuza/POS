from kivy.uix.boxlayout import BoxLayout
from kivymd.uix.label import MDLabel
from kivy.uix.scrollview import ScrollView
from kivymd.uix.list import MDList, OneLineIconListItem, IconLeftWidget, TwoLineIconListItem
from kivy.metrics import dp
from kivy.graphics import Color, Rectangle
from kivymd.uix.button import MDIconButton
from kivymd.uix.label import MDIcon, MDLabel
from kivymd.uix.label.label import  MDIcon

from kivy.core.window import Window
from kivy.utils import get_color_from_hex
from kivy.animation import Animation
from kivymd.uix.expansionpanel import MDExpansionPanel, MDExpansionPanelOneLine
from kivymd.uix.boxlayout import MDBoxLayout
from kivy.uix.behaviors import ButtonBehavior

from components.backend.customExpensionPanel import CustomExpansionPanel
import json
from pathlib import Path
json_file_path = Path('data/app_data/menu_structure.json')


class Sidebar(BoxLayout):
    def __init__(self, switch_view_callback=None, **kwargs):
        # Remove any automatic parent assignment
        kwargs['parent'] = None
        super().__init__(**kwargs)
        self.menu_structure = self.loadJsonFile()
        # Rest of the initialization remains the same

        self.switch_view_callback = switch_view_callback
        self.orientation = 'vertical'
        self.size_hint_x = 0.2
        self.spacing = dp(4)
        self.current_panel = None
        self.title = "MyShop"

        header_layout = BoxLayout(
                orientation='horizontal',
                size_hint_y=0.1,
                padding=(dp(10), dp(10), dp(10), dp(10))
                # Remove padding_x
            )

        icon = MDIcon(
            icon="store",
            theme_text_color="Primary",
            size_hint=(None, None),
            size=(dp(24), dp(24)),
            pos_hint={'center_y': 0.5},
            # padding_x=(dp(10), 0,0,0)
        )
        
        title_item = MDLabel(
            text=self._get_truncated_text(self.title),
            theme_text_color="Primary",
            size_hint_x=1,
            pos_hint={'center_y': 0.5},
            shorten=True,  # Enable text shortening
            shorten_from='right',  # Shorten from right side
            text_size=(None, None),
            # padding_x=(dp(10), 0)
        )
        
        header_layout.add_widget(icon)
        header_layout.add_widget(title_item)

        Window.bind(on_resize=self._on_window_resize)
        self.add_widget(header_layout)
        self.scroll =  ScrollView()
        self.list = MDList(spacing=dp(1.5))

        for menu_title, menu_data in self.menu_structure.items():
            # if menu_data["children"]:
            panel = CustomExpansionPanel(
                title=menu_title,
                icon=menu_data["icon"],
                children=menu_data["children"],
                view_name=menu_data["view"],
                switch_view_callback=self.switch_view_callback,
            )

            
            self.list.add_widget(panel)
        # Add the list to the ScrollView
        self.scroll.add_widget(self.list)
        # Add the header and scroll to the Sidebar
        self.add_widget(self.scroll)

    def switch_view_callback(self, view_name,icon):
        print(f"Sidebar switching to view: {view_name}")
        if hasattr(self.parent, 'switch_view'):
            try:
                self.parent.switch_view(view_name,icon)
            except Exception as e:
                print(f"Error in sidebar switch_view: {e}")
        else:
            print("No switch_view method found in parent")

    def _update_rect(self, instance, value):
        self.rect.pos = instance.pos
        self.rect.size = instance.size

    def _get_truncated_text(self, text):
        window_width = Window.width
        max_chars = int(window_width / 20) 
        return text if len(text) <= max_chars else text[:max_chars] + '..'

    def _on_window_resize(self, *args):
        self.title = self._get_truncated_text(self.title)


    def on_item_click(self, view_name, icon):
        if self.switch_view_callback:
            try:
                self.switch_view_callback(view_name, icon)
            except Exception as e:
                print(f"Error in on_item_click: {e}")
    
    def _update_header_bg(self, instance, value):
        self.header_bg.pos = instance.pos
        self.header_bg.size = instance.size
    
    def loadJsonFile(self):
        if json_file_path.is_file():
            with open(json_file_path, 'r') as file:
                menu_structure_data = json.load(file)
                # Convert JSON arrays back to tuples for children
                for section in menu_structure_data.values():
                    if section["children"]:
                        section["children"] = [tuple(child) for child in section["children"]]
            return menu_structure_data
        else:
            raise FileNotFoundError(f"The JSON file was not found at {json_file_path}")