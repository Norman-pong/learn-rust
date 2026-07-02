# learn-rust 任务运行器
# 用法: just <命令>

# 启动 mdbook 开发服务器
serve:
    mdbook serve --open

# 运行全部练习测试
test:
    cd exercises && cargo test -- --ignored

# 清理构建产物
clean:
    cargo clean
    rm -rf book/

# 生成 weekly 周报模板
weekly:
    @echo "# weekly · $(date +%Y-W%V)" > notes/weekly/$(date +%Y-W%V).md
    @echo "" >> notes/weekly/$(date +%Y-W%V).md
    @echo "## 本周完成" >> notes/weekly/$(date +%Y-W%V).md
    @echo "" >> notes/weekly/$(date +%Y-W%V).md
    @echo "## 卡住什么" >> notes/weekly/$(date +%Y-W%V).md
    @echo "" >> notes/weekly/$(date +%Y-W%V).md
    @echo "## 下周计划" >> notes/weekly/$(date +%Y-W%V).md
    @echo "" >> notes/weekly/$(date +%Y-W%V).md
    @echo "模板已生成: notes/weekly/$(date +%Y-W%V).md"
