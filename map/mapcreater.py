import tkinter as tk

def get_selected_cells():
    selected_cells = []
    for i in range(n):
        for j in range(m):
            if table[i][j]['bg'] == 'red':
                selected_cells.append((j+2, i+3))
    print(selected_cells)
    label.config(text="选中格子数量: {}".format(len(selected_cells)))

def cell_clicked(event):
    cell = event.widget
    if cell['bg'] == 'red':
        cell.configure(bg='white')
    else:
        cell.configure(bg='red')

n = 26  # 行数
m = 63  # 列数

root = tk.Tk()
table = [[None] * m for _ in range(n)]

# 创建显示选中格子数量的标签
label = tk.Label(root, text="选中格子数量: 0", font=("Arial", 14))
label.grid(row=0, columnspan=m)

for i in range(n):
    # 创建显示行号的标签
    row_label = tk.Label(root, text=str(i+3), font=("Arial", 12))
    row_label.grid(row=i+1, column=0, sticky='e')

    for j in range(m):
        if i == 0:
            # 创建显示列号的标签
            col_label = tk.Label(root, text=str(j+2), font=("Arial", 12))
            col_label.grid(row=0, column=j+1)

        cell = tk.Label(root, width=3, height=1, relief=tk.RAISED)
        cell.grid(row=i+1, column=j+1)
        if (i+1) % 2 == 1 or (j+1) % 2 == 1:  # Set odd rows and columns to red
            cell.configure(bg='red')
        cell.bind('<Button-1>', cell_clicked)
        table[i][j] = cell

done_button = tk.Button(root, text="完成", command=get_selected_cells)
done_button.grid(row=n+1, columnspan=m)

root.mainloop()
