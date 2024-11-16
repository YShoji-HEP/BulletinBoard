import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:bulletin_board/messages/all.dart';
import 'package:bulletin_board/common/enums.dart';

class ArchivesPage extends StatelessWidget {
  const ArchivesPage({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    ReqListArchive().sendSignalToRust();
    return StreamBuilder(
      stream: ResListArchive.rustSignalStream,
      builder: (context, snapshot) {
        final received = snapshot.data;
        final List<String> archives;
        if (received == null) {
          archives = [];
        } else {
          archives = received.message.archives;
        }
        return ArchiveContents(archives: archives);
      },
    );
  }
}

class ArchiveContents extends StatefulWidget {
  const ArchiveContents({
    super.key,
    required this.archives,
  });

  final List<String> archives;

  @override
  State<ArchiveContents> createState() => _ArchiveContentsState();
}

class _ArchiveContentsState extends State<ArchiveContents> {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: ArchiveListing(archives: widget.archives),
        ),
        ArchiveController(setState: () => setState(() {}))
      ],
    );
  }
}

class ArchiveListing extends StatelessWidget {
  const ArchiveListing({
    super.key,
    required this.archives,
  });

  final List<String> archives;

  @override
  Widget build(BuildContext context) {
    final settings = Hive.box('settings');
    final sortingOrder =
        SortingOrder.values[settings.get('archiveSortingOrder') ?? 0];

    if (sortingOrder == SortingOrder.ascending) {
      archives.sort((a, b) => a.compareTo(b));
    } else {
      archives.sort((a, b) => b.compareTo(a));
    }

    return Padding(
      padding: const EdgeInsets.all(5.0),
      child: ListView(
        children: archives.map((name) => ArchiveItem(name: name)).toList(),
      ),
    );
  }
}

class ArchiveItem extends StatelessWidget {
  const ArchiveItem({
    super.key,
    required this.name,
  });

  final String name;

  @override
  Widget build(BuildContext context) {
    final nameStyle = Theme.of(context)
        .textTheme
        .displaySmall!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);

    return Card(
      elevation: 2,
      color: Theme.of(context).colorScheme.primaryContainer,
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Row(
          children: [
            Expanded(
              child: Text(
                name,
                style: nameStyle,
              ),
            ),
            Tooltip(
              message: "Load",
              child: OutlinedButton(
                onPressed: () {
                  ReqLoad(acvName: name).sendSignalToRust();
                },
                child: const Icon(Icons.unarchive),
              ),
            ),
            const SizedBox(
              width: 5,
            ),
            Tooltip(
              message: "Restore",
              child: OutlinedButton(
                onPressed: () => showDialog(
                    context: context,
                    builder: (context) => ArchiveRestore(name: name)),
                child: const Icon(Icons.history),
              ),
            ),
            const SizedBox(
              width: 5,
            ),
            Tooltip(
              message: "Rename",
              child: OutlinedButton(
                onPressed: () => showDialog(
                    context: context,
                    builder: (context) => ArchiveRename(name: name)),
                child: const Icon(Icons.edit),
              ),
            ),
            const SizedBox(
              width: 5,
            ),
            Tooltip(
              message: "Delete",
              child: OutlinedButton(
                onPressed: () => showDialog(
                    context: context,
                    builder: (context) => ArchiveDelete(name: name)),
                child: const Icon(Icons.delete),
              ),
            )
          ],
        ),
      ),
    );
  }
}

class ArchiveRestore extends StatelessWidget {
  const ArchiveRestore({
    super.key,
    required this.name,
  });

  final String name;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
        title: Text('Restore: $name'),
        content: const Text(
            'Pressing Ok will remove all the temporary items and restore the data in the archive.'),
        actions: [
          TextButton(
              onPressed: () {
                {
                  Navigator.pop(context);
                }
              },
              child: const Text('Cancel')),
          TextButton(
              onPressed: () {
                {
                  ReqRestore(acvName: name).sendSignalToRust();
                  Navigator.pop(context);
                }
              },
              child: const Text('Ok')),
        ]);
  }
}

class ArchiveRename extends StatelessWidget {
  const ArchiveRename({
    super.key,
    required this.name,
  });

  final String name;

  @override
  Widget build(BuildContext context) {
    final newName = TextEditingController();
    return AlertDialog(
        title: Text('Rename: $name'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text(
                'Choose a new name for the archive. The change will be applied after resetting the server.'),
            TextField(
              decoration: const InputDecoration(hintText: 'New name'),
              controller: newName,
            ),
          ],
        ),
        actions: [
          TextButton(
              onPressed: () {
                {
                  Navigator.pop(context);
                }
              },
              child: const Text('Cancel')),
          TextButton(
              onPressed: () {
                {
                  ReqRenameArchive(acvFrom: name, acvTo: newName.text)
                      .sendSignalToRust();
                  Navigator.pop(context);
                }
              },
              child: const Text('Ok')),
        ]);
  }
}

class ArchiveDelete extends StatelessWidget {
  const ArchiveDelete({
    super.key,
    required this.name,
  });

  final String name;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
        title: Text('Delete: $name'),
        content: const Text(
            'Pressing Ok will delete the archive from the server. The change will be applied after resetting the server.'),
        actions: [
          TextButton(
              onPressed: () {
                {
                  Navigator.pop(context);
                }
              },
              child: const Text('Cancel')),
          TextButton(
              onPressed: () {
                {
                  ReqDeleteArchive(acvName: name).sendSignalToRust();
                  Navigator.pop(context);
                }
              },
              child: const Text('Ok')),
        ]);
  }
}

class ArchiveController extends StatelessWidget {
  const ArchiveController({super.key, required this.setState});

  final Function() setState;

  @override
  Widget build(BuildContext context) {
    final settings = Hive.box('settings');
    final sortingOrder =
        SortingOrder.values[settings.get('archiveSortingOrder') ?? 0];
    return BottomAppBar(
        height: 40,
        color: Theme.of(context).colorScheme.secondary,
        padding: const EdgeInsets.only(top: 8, bottom: 8, right: 10),
        child: Row(mainAxisAlignment: MainAxisAlignment.end, children: [
          FloatingActionButton.small(
            tooltip: 'Sort Direction',
            onPressed: () {
              settings.put(
                  'archiveSortingOrder',
                  sortingOrder == SortingOrder.ascending
                      ? SortingOrder.descending.index
                      : SortingOrder.ascending.index);
              setState();
            },
            child: Icon(sortingOrder == SortingOrder.ascending
                ? Icons.arrow_downward
                : Icons.arrow_upward),
          ),
          const SizedBox(width: 20),
          FloatingActionButton(
            tooltip: 'Refresh',
            onPressed: () {
              ReqListArchive().sendSignalToRust();
            },
            child: const Icon(Icons.refresh),
          ),
          const SizedBox(width: 20),
          FloatingActionButton(
            tooltip: 'Reset',
            onPressed: () => showDialog(
                context: context,
                builder: (context) => AlertDialog(
                        title: const Text('Reset'),
                        content: const Text(
                            'Pressing Ok will reset the server and erase all the temporary items.'),
                        actions: [
                          TextButton(
                              onPressed: () {
                                {
                                  Navigator.pop(context);
                                }
                              },
                              child: const Text('Cancel')),
                          TextButton(
                              onPressed: () {
                                {
                                  ReqReset().sendSignalToRust();
                                  ReqListArchive().sendSignalToRust();
                                  Navigator.pop(context);
                                }
                              },
                              child: const Text('Ok')),
                        ])),
            child: const Icon(Icons.reset_tv),
          ),
          const SizedBox(width: 3),
        ]));
  }
}
