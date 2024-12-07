import 'dart:io';
import 'package:flutter/material.dart';
import 'package:human_file_size/human_file_size.dart';
import 'package:fixnum/fixnum.dart';
import 'package:bulletin_board/messages/all.dart';

class ServerPage extends StatefulWidget {
  const ServerPage({
    super.key,
  });

  @override
  State<ServerPage> createState() => _ServerPageState();
}

class _ServerPageState extends State<ServerPage> {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const SizedBox(
          height: 20,
        ),
        const ServerStatus(),
        const Expanded(child: ServerLog()),
        ServerController(setState: () => setState(() {}))
      ],
    );
  }
}

class ServerStatus extends StatelessWidget {
  const ServerStatus({super.key});

  @override
  Widget build(BuildContext context) {
    ReqStatus().sendSignalToRust();

    return StreamBuilder(
        stream: ResStatus.rustSignalStream,
        builder: (context, snapshot) {
          final received = snapshot.data;

          final String totalDatasize;
          final String memoryUsed;
          final String memoryUsedPercentage;
          final Int64 bulletins;
          final Int64 files;
          final Int64 archives;

          if (received == null) {
            totalDatasize = "-";
            memoryUsed = "-";
            memoryUsedPercentage = "-";
            bulletins = Int64.ZERO;
            files = Int64.ZERO;
            archives = Int64.ZERO;
          } else {
            totalDatasize =
                humanFileSize(received.message.totalDatasize.toInt());
            memoryUsed = humanFileSize(received.message.memoryUsed.toInt());
            memoryUsedPercentage =
                '${received.message.memoryUsedPercentage.toStringAsPrecision(3).toString()} %';
            bulletins = received.message.bulletins;
            files = received.message.files;
            archives = received.message.archives;
          }

          final sectionStyle = Theme.of(context).textTheme.bodyLarge!.copyWith(
              color: Theme.of(context).colorScheme.onPrimaryContainer);

          return Padding(
            padding: const EdgeInsets.symmetric(horizontal: 20),
            child: Column(
              children: [
                Text(
                  "Status",
                  style: sectionStyle,
                ),
                SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: DataTable(
                      border: TableBorder(
                          horizontalInside: BorderSide(
                              color: Theme.of(context).colorScheme.onSurface)),
                      columns: const [
                        DataColumn(label: Text('total data size')),
                        DataColumn(label: Text('memory used')),
                        DataColumn(label: Text('memory used (%)')),
                        DataColumn(label: Text('#items')),
                        DataColumn(label: Text('#files')),
                        DataColumn(label: Text('#archived items')),
                      ],
                      rows: [
                        DataRow(cells: [
                          DataCell(Text(totalDatasize)),
                          DataCell(Text(memoryUsed)),
                          DataCell(Text(memoryUsedPercentage)),
                          DataCell(Text('$bulletins')),
                          DataCell(Text('$files')),
                          DataCell(Text('$archives')),
                        ]),
                      ]),
                ),
              ],
            ),
          );
        });
  }
}

class ServerLog extends StatelessWidget {
  const ServerLog({super.key});

  @override
  Widget build(BuildContext context) {
    ReqLog().sendSignalToRust();
    final sectionStyle = Theme.of(context)
        .textTheme
        .bodyLarge!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);
    return StreamBuilder(
        stream: ResLog.rustSignalStream,
        builder: (context, snapshot) {
          final received = snapshot.data;
          final String log;
          if (received == null) {
            log = "";
          } else {
            log = received.message.log;
          }
          return Padding(
            padding: const EdgeInsets.all(20.0),
            child: Column(
              children: [
                Text(
                  "Log",
                  style: sectionStyle,
                ),
                Expanded(
                    child: Container(
                        width: double.infinity,
                        decoration: BoxDecoration(
                            border: Border.all(),
                            color: Theme.of(context).colorScheme.surface),
                        child: SelectableText(log,
                            style: TextStyle(
                                color:
                                    Theme.of(context).colorScheme.onSurface))))
              ],
            ),
          );
        });
  }
}

class ServerController extends StatelessWidget {
  const ServerController({super.key, required this.setState});

  final Function() setState;

  @override
  Widget build(BuildContext context) {
    return BottomAppBar(
        height: 40,
        color: Theme.of(context).colorScheme.secondary,
        padding: const EdgeInsets.only(top: 8, bottom: 8, right: 10),
        child: Row(mainAxisAlignment: MainAxisAlignment.end, children: [
          FloatingActionButton(
            tooltip: 'Refresh',
            onPressed: () {
              ReqLog().sendSignalToRust();
            },
            child: const Icon(Icons.refresh),
          ),
          const SizedBox(width: 20),
          FloatingActionButton(
            tooltip: 'Clear',
            onPressed: () {
              ReqClearLog().sendSignalToRust();
              sleep(const Duration(milliseconds: 10));
              ReqLog().sendSignalToRust();
            },
            child: const Icon(Icons.delete),
          ),
          const SizedBox(width: 3),
        ]));
  }
}
