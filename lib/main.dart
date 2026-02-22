import 'dart:convert';
import 'dart:math';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;

void main() {
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
  List<Task> tasks = [];

  void getAllTasks() => Future.sync(() async {
    final result = await http.get(Uri.parse('http://127.0.0.1:8080/tasks'));
    final tasks = (jsonDecode(result.body) as List<dynamic>)
        .map((e) => Task.fromJson(e as Map<String, dynamic>))
        .toList();
    this.tasks = tasks;
    setState(() {
      this.tasks.sort((a, b) => a.id.compareTo(b.id));
    });
  });

  void createTask() => Future.sync(() async {
    final maxId = tasks.isEmpty ? 0 : tasks.map((task) => task.id).reduce(max);
    final result = await http.post(
      Uri.parse('http://127.0.0.1:8080/tasks'),
      body: jsonEncode({
        'id': 0,
        'title': 'Task ${maxId + 1}',
        'description': 'Description ${maxId + 1}',
        'completed': false,
      }),
      headers: {'Content-Type': 'application/json'},
    );
    print(result.statusCode);
    getAllTasks();
  });

  void updateTask(int id, bool completed) => Future.sync(() async {
    final task = tasks
        .firstWhere((task) => task.id == id)
        .copyWith(completed: completed);
    await http.put(
      Uri.parse('http://127.0.0.1:8080/tasks/$id'),
      body: task.toJson(),
      headers: {'Content-Type': 'application/json'},
    );
    final index = tasks.indexWhere((task) => task.id == id);
    setState(() {
      tasks[index] = task;
    });
  });

  void deleteTask(int id) => Future.sync(() async {
    await http.delete(Uri.parse('http://127.0.0.1:8080/tasks/$id'));
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

class Task {
  final int id;
  final String title;
  final String description;
  final bool completed;

  const Task({
    required this.id,
    required this.title,
    required this.description,
    required this.completed,
  });

  factory Task.fromJson(Map<String, dynamic> json) {
    return Task(
      id: json['id']! as int,
      title: json['title']! as String,
      description: json['description']! as String,
      completed: json['completed']! as bool,
    );
  }

  String toJson() {
    return jsonEncode({
      'id': id,
      'title': title,
      'description': description,
      'completed': completed,
    });
  }

  Task copyWith({required bool completed}) {
    return Task(
      id: id,
      title: title,
      description: description,
      completed: completed,
    );
  }
}
