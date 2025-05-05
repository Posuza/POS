from kivy.uix.scrollview import ScrollView
from kivy.uix.boxlayout import BoxLayout
from kivy.uix.gridlayout import GridLayout
from kivy.metrics import dp
from kivymd.uix.label import MDLabel
from kivy.graphics import Color, RoundedRectangle, Line, Ellipse
from kivymd.uix.card import MDCard
from math import cos, sin, pi
from components.backend.dashborad_view.line_chart import LineChartWidget  # Fixed spelling
from components.backend.dashborad_view.pie_chart import PieChartWidget  # Fixed spelling


class DashboardView(ScrollView):
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.bar_width = 0
        
        self.container = BoxLayout(
            orientation='vertical',
            padding=[15, 5, 15, 5],
            spacing=10,
            size_hint_y=None
        )
        self.container.bind(minimum_height=self.container.setter('height'))

        # First box with horizontal ScrollView
        self.box1 = BoxLayout(
            size_hint_y=None, 
            height=dp(150),
            # padding=[0, 10, 0, 10]
        )
        
        self.horizontal_scroll = ScrollView(
            do_scroll_y=False,
            bar_width=0
        )
        
        self.horizontal_container = BoxLayout(
            orientation='horizontal',
            spacing=10,
            size_hint_x=None
        )
        self.horizontal_container.bind(minimum_width=self.horizontal_container.setter('width'))

        # Add 6 boxes to horizontal scroll
        for i in range(6):
            small_box = BoxLayout(
                size_hint_x=None, 
                width=dp(300)
            )
            with small_box.canvas.before:
                Color(0.8, 0.2 + i/10, 0.2, 1)
                bg = RoundedRectangle(pos=small_box.pos, size=small_box.size, radius=[10])
                small_box.bind(pos=lambda inst, val, bg=bg: setattr(bg, 'pos', val))
                small_box.bind(size=lambda inst, val, bg=bg: setattr(bg, 'size', val))
            self.horizontal_container.add_widget(small_box)

        self.horizontal_scroll.add_widget(self.horizontal_container)
        self.box1.add_widget(self.horizontal_scroll)

        # Second box with GridLayout
        self.box2 = BoxLayout(
            size_hint_y=None
        )

        # Create responsive GridLayout
        self.grid = GridLayout(
            rows=1,  # Start with 1 row
            spacing=10, 
            padding=[0, 0, 0, 0],
            size_hint=(1, None)
        )

        # Create 4 responsive grid boxes

        for i in range(2):
            grid_box = BoxLayout(
                size_hint=(1, None),
                height=dp(300),
                size_hint_min_x=dp(250)
            )
            if i == 0:  # Changed to i == 2 for the third box
                pieChart = PieChartWidget()
                grid_box.add_widget(pieChart)
            with grid_box.canvas.before:
                Color(0.3 + 0, 0.5, 0.7, 1)
                bg = RoundedRectangle(pos=grid_box.pos, size=grid_box.size, radius=[15])
                grid_box.bind(pos=lambda inst, val, bg=bg: setattr(bg, 'pos', val))
                grid_box.bind(size=lambda inst, val, bg=bg: setattr(bg, 'size', val))
            self.grid.add_widget(grid_box)

        # Add GridLayout to box2
        self.box2.add_widget(self.grid)

        # Last box
        self.lineChartbox = BoxLayout(
            size_hint=(1, None),
            size_hint_min_x=dp(250)
        )

        # Add the line chart widget
        historyChart = LineChartWidget()
        self.lineChartbox.add_widget(historyChart)

        # Bind the box height to its children
        self.lineChartbox.bind(minimum_height=self.lineChartbox.setter('height'))

        # Add background styling
        with self.lineChartbox.canvas.before:
            Color(0.3 + 0/10, 0.5, 0.7, 1)
            bg = RoundedRectangle(pos=self.lineChartbox.pos, size=self.lineChartbox.size, radius=[10])
            self.lineChartbox.bind(pos=lambda inst, val, bg=bg: setattr(bg, 'pos', val))
            self.lineChartbox.bind(size=lambda inst, val, bg=bg: setattr(bg, 'size', val))


        self.container.add_widget(self.box1)
        self.container.add_widget(self.lineChartbox)
        self.container.add_widget(self.box2)
        
        self.add_widget(self.container)

        # Bind to window size changes
        self.bind(size=self._update_layout)
        
        # Bind grid height to its children
        self.grid.bind(children=self._update_grid_height)
        self.box2.bind(minimum_height=self.box2.setter('height'))

    def _update_layout(self, instance, value):
        width = value[0]
        if width < dp(450):
            self.grid.rows = 4
            self.grid.cols = 1
        elif width < dp(800):
            self.grid.rows = 2
            self.grid.cols = 2
        else:
            self.grid.rows = 1
            self.grid.cols = 4
        self._update_grid_height()

    def _update_grid_height(self, *args):
        rows = self.grid.rows
        child_height = dp(300)  # Match the new grid_box height
        spacing_height = (rows - 1) * self.grid.spacing[1]
        padding_height = self.grid.padding[1] + self.grid.padding[3]
        total_height = (rows * child_height) + spacing_height + padding_height
        
        self.grid.height = total_height
        self.box2.height = total_height
