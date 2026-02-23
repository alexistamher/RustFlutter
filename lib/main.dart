import 'dart:math';
import 'package:flutter/material.dart';
import 'package:tasks_plugin/tasks_plugin.dart' as tasks_plugin;

void main() {
  tasks_plugin.RustLib.init();
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MaterialApp(
      home: Scaffold(body: Center(child: _CounterWidget())),
    );
  }
}

class _CounterWidget extends StatefulWidget {
  const _CounterWidget();

  @override
  State<_CounterWidget> createState() => _CounterWidgetState();
}

class _CounterWidgetState extends State<_CounterWidget> {
  List<tasks_plugin.Task> tasks = [];

  void getAllTasks() => Future.sync(() async {
    tasks = await tasks_plugin.getAllTasks();
    setState(() {
      tasks.sort((a, b) => a.id.compareTo(b.id));
    });
  });

  void createTask() => Future.sync(() async {
    final maxId = tasks.isEmpty ? 0 : tasks.map((task) => task.id).reduce(max);
    await tasks_plugin.createTask(
      id: maxId + 1,
      title: 'Task ${maxId + 1}',
      description: 'Description ${maxId + 1}',
      completed: false,
    );
    getAllTasks();
  });

  void updateTask(int id, bool completed) => Future.sync(() async {
    final task = await tasks
        .firstWhere((task) => task.id == id)
        .copyWith(completed: completed);
    await tasks_plugin.updateTask(
      id: id,
      title: task.title,
      description: task.description,
      completed: task.completed,
    );
    final index = tasks.indexWhere((task) => task.id == id);
    setState(() {
      tasks[index] = task;
    });
  });

  void deleteTask(int id) => Future.sync(() async {
    await tasks_plugin.deleteTask(id: id);
    setState(() {
      tasks.removeWhere((task) => task.id == id);
    });
  });

  @override
  void initState() {
    super.initState();
    getAllTasks();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      floatingActionButton: FloatingActionButton(
        onPressed: createTask,
        child: const Icon(Icons.add),
      ),
      body: ListView.builder(
        itemCount: tasks.length,
        itemBuilder: (context, index) {
          final task = tasks[index];
          return Column(
            children: [
              ListTile(
                title: Text(task.title),
                subtitle: Text(task.description),
                trailing: IconButton(
                  onPressed: () => deleteTask(task.id),
                  icon: Icon(Icons.delete),
                ),
              ),
              Align(
                alignment: Alignment.centerRight,
                child: task.completed
                    ? Container(
                        margin: const EdgeInsets.symmetric(
                          vertical: 8.0,
                        ).copyWith(right: 16.0),
                        decoration: BoxDecoration(
                          color: Colors.green.shade100,
                          borderRadius: BorderRadius.circular(8),
                        ),
                        padding: const EdgeInsets.symmetric(
                          horizontal: 8.0,
                          vertical: 4.0,
                        ),
                        child: Text('Completed'),
                      )
                    : Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 4.0),
                        child: Row(
                          mainAxisSize: .min,
                          children: [
                            Text('Complete'),
                            Checkbox(
                              value: task.completed,
                              onChanged: (value) => updateTask(task.id, value!),
                            ),
                          ],
                        ),
                      ),
              ),
            ],
          );
        },
      ),
    );
  }
}
