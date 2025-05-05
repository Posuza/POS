from kivy.uix.widget import Widget
from kivy.graphics import Color, Ellipse, Line
from kivy.properties import ListProperty, NumericProperty
from math import cos, sin, pi

class PieChartWidget(Widget):
    data = ListProperty([30, 30, 40])  # Example data percentages
    colors = ListProperty([
        (0.2, 0.7, 0.3, 1),  # Green
        (0.3, 0.5, 0.8, 1),  # Blue
        (0.8, 0.3, 0.3, 1)   # Red
    ])
    
    def __init__(self, **kwargs):
        super().__init__(**kwargs)
        self.bind(pos=self.draw_pie, size=self.draw_pie)
        
    def draw_pie(self, *args):
        self.canvas.clear()
        center_x = self.center_x
        center_y = self.center_y
        radius = min(self.width, self.height) * 0.4
        
        start_angle = 0
        for i, percentage in enumerate(self.data):
            with self.canvas:
                Color(*self.colors[i])
                angle = percentage * 3.6  # Convert percentage to degrees
                end_angle = start_angle + angle
                
                # Convert angles to radians for calculations
                start_rad = start_angle * pi / 180
                end_rad = end_angle * pi / 180
                
                # Calculate points for the triangle
                points = [
                    center_x, center_y,
                    center_x + radius * cos(start_rad), center_y + radius * sin(start_rad),
                    center_x + radius * cos(end_rad), center_y + radius * sin(end_rad)
                ]
                
                # Draw the sector
                Ellipse(
                    pos=(center_x - radius, center_y - radius),
                    size=(radius * 2, radius * 2),
                    angle_start=start_angle,
                    angle_end=end_angle
                )
                
            start_angle = end_angle
