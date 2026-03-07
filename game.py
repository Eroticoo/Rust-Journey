#!/usr/bin/env python3
"""
贪吃蛇小游戏 - 使用 tkinter 实现
控制: 上下左右方向键移动蛇，空格键重新开始
"""

import tkinter as tk
import random
from tkinter import messagebox


class SnakeGame:
    def __init__(self, master):
        self.master = master
        self.master.title("贪吃蛇游戏")
        self.master.resizable(False, False)

        # 游戏参数
        self.cell_size = 20
        self.width = 400
        self.height = 400
        self.canvas = tk.Canvas(
            master, width=self.width, height=self.height, bg="black"
        )
        self.canvas.pack()

        # 游戏状态
        self.snake = []
        self.food = None
        self.direction = "right"
        self.score = 0
        self.game_over = False
        self.game_running = False

        # 计分标签
        self.score_label = tk.Label(
            master, text=f"分数: {self.score}", font=("Arial", 14)
        )
        self.score_label.pack(pady=5)

        # 开始按钮
        self.start_button = tk.Button(
            master, text="开始游戏", command=self.start_game, font=("Arial", 12)
        )
        self.start_button.pack(pady=5)

        # 绑定键盘事件
        self.master.bind("<Key>", self.on_key_press)

        # 绘制初始界面
        self.draw_welcome()

    def draw_welcome(self):
        """绘制欢迎界面"""
        self.canvas.create_text(
            self.width / 2,
            self.height / 2 - 20,
            text="贪吃蛇游戏",
            fill="white",
            font=("Arial", 20)
        )
        self.canvas.create_text(
            self.width / 2,
            self.height / 2 + 20,
            text="按方向键控制移动",
            fill="white",
            font=("Arial", 12)
        )

    def start_game(self):
        """开始游戏"""
        self.snake = [
            [self.width / 2, self.height / 2],
            [self.width / 2, self.height / 2 + self.cell_size],
            [self.width / 2, self.height / 2 + 2 * self.cell_size]
        ]
        self.direction = "right"
        self.score = 0
        self.game_over = False
        self.game_running = True
        self.score_label.config(text=f"分数: {self.score}")
        self.start_button.config(state="disabled")
        self.spawn_food()
        self.game_loop()

    def spawn_food(self):
        """生成食物"""
        x = random.randint(0, (self.width - self.cell_size) // self.cell_size) * self.cell_size
        y = random.randint(0, (self.height - self.cell_size) // self.cell_size) * self.cell_size
        self.food = [x, y]

    def on_key_press(self, event):
        """处理键盘事件"""
        if not self.game_running and not self.game_over:
            return

        key = event.keysym

        # 游戏结束按空格重新开始
        if self.game_over and key == "space":
            self.start_game()
            return

        # 方向控制（禁止反向移动）
        if key == "Up" and self.direction != "down":
            self.direction = "up"
        elif key == "Down" and self.direction != "up":
            self.direction = "down"
        elif key == "Left" and self.direction != "right":
            self.direction = "left"
        elif key == "Right" and self.direction != "left":
            self.direction = "right"
        elif key == "space" and self.game_over:
            self.start_game()

    def move_snake(self):
        """移动蛇"""
        head = self.snake[0].copy()

        if self.direction == "up":
            head[1] -= self.cell_size
        elif self.direction == "down":
            head[1] += self.cell_size
        elif self.direction == "left":
            head[0] -= self.cell_size
        elif self.direction == "right":
            head[0] += self.cell_size

        self.snake.insert(0, head)

        # 检查是否吃到食物
        if head == self.food:
            self.score += 1
            self.score_label.config(text=f"分数: {self.score}")
            self.spawn_food()
        else:
            self.snake.pop()

    def check_collision(self):
        """检查碰撞"""
        head = self.snake[0]

        # 检查墙壁碰撞
        if (
            head[0] < 0
            or head[0] >= self.width
            or head[1] < 0
            or head[1] >= self.height
        ):
            return True

        # 检查自身碰撞
        if head in self.snake[1:]:
            return True

        return False

    def draw(self):
        """绘制游戏画面"""
        self.canvas.delete("all")

        # 绘制蛇
        for i, segment in enumerate(self.snake):
            color = "green" if i == 0 else "lightgreen"
            self.canvas.create_rectangle(
                segment[0],
                segment[1],
                segment[0] + self.cell_size,
                segment[1] + self.cell_size,
                fill=color,
                outline="white"
            )

        # 绘制食物
        self.canvas.create_oval(
            self.food[0],
            self.food[1],
            self.food[0] + self.cell_size,
            self.food[1] + self.cell_size,
            fill="red",
            outline="white"
        )

    def game_over_screen(self):
        """游戏结束画面"""
        self.canvas.create_text(
            self.width / 2,
            self.height / 2 - 20,
            text="游戏结束!",
            fill="red",
            font=("Arial", 20)
        )
        self.canvas.create_text(
            self.width / 2,
            self.height / 2 + 20,
            text=f"最终分数: {self.score}",
            fill="white",
            font=("Arial", 14)
        )
        self.canvas.create_text(
            self.width / 2,
            self.height / 2 + 50,
            text="按空格键重新开始",
            fill="white",
            font=("Arial", 12)
        )

    def game_loop(self):
        """游戏主循环"""
        if not self.game_running:
            return

        self.move_snake()

        if self.check_collision():
            self.game_over = True
            self.game_running = False
            self.game_over_screen()
            self.start_button.config(state="normal")
        else:
            self.draw()
            self.master.after(100, self.game_loop)


def main():
    """主函数"""
    root = tk.Tk()
    game = SnakeGame(root)
    root.mainloop()


if __name__ == "__main__":
    main()
