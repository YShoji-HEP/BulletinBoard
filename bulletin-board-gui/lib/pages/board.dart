import 'dart:io';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:hive/hive.dart';
import 'package:popover/popover.dart';
import 'package:collection/collection.dart';
import 'package:fixnum/fixnum.dart';
import 'package:human_file_size/human_file_size.dart';
import 'package:bulletin_board/common/enums.dart';
import 'package:bulletin_board/messages/all.dart';

class BoardPage extends StatelessWidget {
  const BoardPage({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    ReqViewBoard().sendSignalToRust();
    return StreamBuilder(
      stream: ResViewBoard.rustSignalStream,
      builder: (context, snapshot) {
        final received = snapshot.data;
        final List<ResBulletinItem> bulletins;
        if (received == null) {
          bulletins = [];
        } else {
          bulletins = received.message.bulletins;
        }

        return BoardContents(bulletins: bulletins);
      },
    );
  }
}

class BoardContents extends StatefulWidget {
  const BoardContents({
    super.key,
    required this.bulletins,
  });

  final List<ResBulletinItem> bulletins;

  @override
  State<BoardContents> createState() => _BoardContentsState();
}

class _BoardContentsState extends State<BoardContents> {
  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: BoardListing(bulletins: widget.bulletins),
        ),
        BoardController(setState: () => setState(() {}))
      ],
    );
  }
}

class BoardListing extends StatelessWidget {
  const BoardListing({
    super.key,
    required this.bulletins,
  });

  final List<ResBulletinItem> bulletins;

  @override
  Widget build(BuildContext context) {
    final settings = Hive.box('settings');
    final columns = settings.get('boardColumnNum') ?? 3;
    final sortingField =
        SortingField.values[settings.get('boardSortingField') ?? 0];
    final sortingOrder =
        SortingOrder.values[settings.get('boardSortingOrder') ?? 0];

    if (sortingField == SortingField.title) {
      if (sortingOrder == SortingOrder.ascending) {
        bulletins.sort((a, b) => a.title.compareTo(b.title));
      } else {
        bulletins.sort((a, b) => b.title.compareTo(a.title));
      }
    } else {
      if (sortingOrder == SortingOrder.ascending) {
        bulletins.sort((a, b) => a.tag.compareTo(b.tag));
      } else {
        bulletins.sort((a, b) => b.tag.compareTo(a.tag));
      }
    }

    final chunks = bulletins
        .map((item) => Expanded(
              child: BulletinItem(
                title: item.title,
                tag: item.tag,
                revisions: item.revisions,
                requireTag: item.requireTag,
              ),
            ))
        .slices(columns)
        .toList();
    for (final elem in chunks) {
      final space = columns - elem.length;
      for (int i = 0; i < space; i++) {
        elem.add(const Expanded(child: SizedBox()));
      }
    }

    return Padding(
        padding: const EdgeInsets.only(right: 10),
        child: ListView.builder(
            itemCount: chunks.length,
            itemBuilder: (context, index) {
              return Row(
                children: chunks[index],
              );
            }));
  }
}

class BulletinItem extends StatelessWidget {
  const BulletinItem({
    super.key,
    required this.title,
    required this.tag,
    required this.revisions,
    required this.requireTag,
  });

  final String title;
  final String tag;
  final Int64 revisions;
  final bool requireTag;

  @override
  Widget build(BuildContext context) {
    final settings = Hive.box('settings');
    final targetLanguage =
        TargetLanguage.values[settings.get('targetLanguage') ?? 0];
    // final clickAction =
    //     BoardClickAction.values[settings.get('boardClickAction') ?? 0];
    bool copyFull = settings.get('copyFull') ?? false;
    var pythonAlias = settings.get('pythonAlias') ?? '';
    if (pythonAlias == '') {
      pythonAlias = 'bulletin_board_client';
    }

    final titleStyle = Theme.of(context)
        .textTheme
        .displaySmall!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);
    final tagStyle = Theme.of(context)
        .textTheme
        .bodySmall!
        .copyWith(color: Theme.of(context).colorScheme.onSecondary);
    final revStyle = Theme.of(context)
        .textTheme
        .bodyLarge!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);

    return Padding(
      padding: const EdgeInsets.only(top: 10, left: 10),
      child: Material(
        elevation: 2,
        borderRadius: BorderRadius.circular(12),
        color: Theme.of(context).colorScheme.primaryContainer,
        child: InkWell(
          borderRadius: BorderRadius.circular(12),
          onTap: () {
            String text;
            if (requireTag || copyFull) {
              switch (targetLanguage) {
                case TargetLanguage.mathematica:
                  {
                    text = 'BBRead["$title","$tag"]';
                  }
                case TargetLanguage.python:
                  {
                    text = '$pythonAlias.read("$title","$tag")';
                  }
              }
            } else {
              switch (targetLanguage) {
                case TargetLanguage.mathematica:
                  {
                    text = 'BBRead["$title"]';
                  }
                case TargetLanguage.python:
                  {
                    text = '$pythonAlias.read("$title")';
                  }
              }
            }
            Clipboard.setData(ClipboardData(text: text));
            // if (clickAction == BoardClickAction.clipboard) {
            //   Clipboard.setData(ClipboardData(text: text));
            // } else {
            //   ReqKeyInput(text: text).sendSignalToRust();
            // }
          },
          onSecondaryTap: () => showPopover(
              direction: PopoverDirection.bottom,
              arrowHeight: 0,
              arrowWidth: 0,
              context: context,
              bodyBuilder: (_) => BulletinPopup(title: title, tag: tag),
              width: 250,
              height: 50),
          child: Column(children: [
            Text(
              title,
              style: titleStyle,
            ),
            const SizedBox(height: 5),
            Container(
                width: double.infinity,
                color: Theme.of(context).colorScheme.secondary,
                child: Center(child: Text(tag, style: tagStyle))),
            const SizedBox(height: 3),
            Row(mainAxisSize: MainAxisSize.min, children: [
              const Icon(Icons.description),
              Text(
                ':$revisions',
                style: revStyle,
              )
            ]),
            const SizedBox(height: 3),
          ]),
        ),
      ),
    );
  }
}

class BulletinPopup extends StatelessWidget {
  const BulletinPopup({
    super.key,
    required this.title,
    required this.tag,
  });

  final String title;
  final String tag;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 10),
      child: Row(
        children: [
          Tooltip(
            message: 'Info',
            child: TextButton(
                onPressed: () {
                  showDialog(
                      context: context,
                      builder: (context) =>
                          BulletinInfo(title: title, tag: tag));
                },
                child: const Icon(Icons.article)),
          ),
          Tooltip(
            message: 'Relabel',
            child: TextButton(
                onPressed: () => showDialog(
                    context: context,
                    builder: (context) =>
                        BulletinRelabel(title: title, tag: tag)),
                child: const Icon(Icons.edit)),
          ),
          Tooltip(
            message: 'Archive',
            child: TextButton(
                onPressed: () => showDialog(
                    context: context,
                    builder: (context) =>
                        BulletinArchive(title: title, tag: tag)),
                child: const Icon(Icons.archive)),
          ),
          Tooltip(
            message: 'Remove',
            child: TextButton(
                onPressed: () => showDialog(
                    context: context,
                    builder: (context) =>
                        BulletinRemove(title: title, tag: tag)),
                child: const Icon(Icons.delete)),
          ),
        ],
      ),
    );
  }
}

class BulletinInfo extends StatelessWidget {
  const BulletinInfo({
    super.key,
    required this.title,
    required this.tag,
  });

  final String title;
  final String tag;

  @override
  Widget build(BuildContext context) {
    ReqGetInfo(title: title, tag: tag).sendSignalToRust();

    return StreamBuilder(
      stream: ResGetInfo.rustSignalStream,
      builder: (context, snapshot) {
        final received = snapshot.data;
        List<DataRow> table;
        if (received == null) {
          table = [];
        } else {
          table = received.message.info.map((i) {
            final datasize = humanFileSize(i.datasize.toInt());
            return DataRow(cells: [
              DataCell(Text('${i.revision}')),
              DataCell(Text(datasize)),
              DataCell(Text(i.timestamp)),
              DataCell(Text(i.backend)),
            ]);
          }).toList();
        }

        return Card(
          child: Column(
            children: [
              const SizedBox(
                height: 10,
              ),
              OutlinedButton(
                  onPressed: () {
                    Navigator.pop(context);
                    Navigator.pop(context);
                  },
                  child: const Icon(Icons.close)),
              Expanded(
                child: SingleChildScrollView(
                  scrollDirection: Axis.horizontal,
                  child: SingleChildScrollView(
                    scrollDirection: Axis.vertical,
                    child: DataTable(columns: const [
                      DataColumn(label: Text('Revision number')),
                      DataColumn(label: Text('Data size')),
                      DataColumn(label: Text('Timestamp')),
                      DataColumn(label: Text('Backend')),
                    ], rows: table),
                  ),
                ),
              )
            ],
          ),
        );
      },
    );
  }
}

class BulletinRelabel extends StatelessWidget {
  const BulletinRelabel({
    super.key,
    required this.title,
    required this.tag,
  });

  final String title;
  final String tag;

  @override
  Widget build(BuildContext context) {
    final newTitle = TextEditingController();
    final newTag = TextEditingController();

    return AlertDialog(
        title: Text('Relabel: $title, $tag'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text(
                'Choose a new name and/or a new tag for the bulletin. Leave blank if not changing.'),
            TextField(
              decoration: const InputDecoration(hintText: 'New title'),
              controller: newTitle,
            ),
            TextField(
              decoration: const InputDecoration(hintText: 'New tag'),
              controller: newTag,
            ),
          ],
        ),
        actions: [
          TextButton(
              onPressed: () {
                {
                  Navigator.pop(context);
                  Navigator.pop(context);
                }
              },
              child: const Text('Cancel')),
          TextButton(
              onPressed: () {
                {
                  ReqRelabel(
                          titleFrom: title,
                          tagFrom: tag,
                          titleTo: newTitle.text,
                          tagTo: newTag.text)
                      .sendSignalToRust();
                  Navigator.pop(context);
                  Navigator.pop(context);
                  sleep(const Duration(milliseconds: 10));
                  ReqViewBoard().sendSignalToRust();
                }
              },
              child: const Text('Ok')),
        ]);
  }
}

class BulletinArchive extends StatelessWidget {
  const BulletinArchive({
    super.key,
    required this.title,
    required this.tag,
  });

  final String title;
  final String tag;

  @override
  Widget build(BuildContext context) {
    final archiveName = TextEditingController();

    return AlertDialog(
        title: Text('Archive: $title, $tag'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text('Choose a name of the archive for the bulletin.'),
            TextField(
              decoration: const InputDecoration(hintText: 'Archive name'),
              controller: archiveName,
            ),
          ],
        ),
        actions: [
          TextButton(
              onPressed: () {
                {
                  Navigator.pop(context);
                  Navigator.pop(context);
                }
              },
              child: const Text('Cancel')),
          TextButton(
              onPressed: () {
                {
                  ReqArchive(
                    acvName: archiveName.text,
                    title: title,
                    tag: tag,
                  ).sendSignalToRust();
                  Navigator.pop(context);
                  Navigator.pop(context);
                  sleep(const Duration(milliseconds: 10));
                  ReqViewBoard().sendSignalToRust();
                }
              },
              child: const Text('Ok')),
        ]);
  }
}

class BulletinRemove extends StatelessWidget {
  const BulletinRemove({
    super.key,
    required this.title,
    required this.tag,
  });

  final String title;
  final String tag;

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
        title: const Text('Remove'),
        content:
            Text('Pressing Ok will remove ($title, $tag) from the server.'),
        actions: [
          TextButton(
              onPressed: () {
                {
                  Navigator.pop(context);
                  Navigator.pop(context);
                }
              },
              child: const Text('Cancel')),
          TextButton(
              onPressed: () {
                {
                  ReqRemove(title: title, tag: tag).sendSignalToRust();
                  Navigator.pop(context);
                  Navigator.pop(context);
                  sleep(const Duration(milliseconds: 10));
                  ReqViewBoard().sendSignalToRust();
                }
              },
              child: const Text('Ok')),
        ]);
  }
}

class BoardController extends StatelessWidget {
  const BoardController({super.key, required this.setState});

  final Function() setState;

  @override
  Widget build(BuildContext context) {
    final settings = Hive.box('settings');
    final sortingField =
        SortingField.values[settings.get('boardSortingField') ?? 0];
    final sortingOrder =
        SortingOrder.values[settings.get('boardSortingOrder') ?? 0];
    final archiveName = TextEditingController();
    return BottomAppBar(
        height: 40,
        color: Theme.of(context).colorScheme.secondary,
        padding: const EdgeInsets.only(top: 8, bottom: 8, right: 10),
        child: Row(mainAxisAlignment: MainAxisAlignment.end, children: [
          FloatingActionButton(
            tooltip: 'Sort Field',
            onPressed: () {
              settings.put(
                  'boardSortingField',
                  sortingField == SortingField.title
                      ? SortingField.tag.index
                      : SortingField.title.index);
              setState();
            },
            child: Text(sortingField.label),
          ),
          const SizedBox(width: 3),
          FloatingActionButton.small(
            tooltip: 'Sort Direction',
            onPressed: () {
              settings.put(
                  'boardSortingOrder',
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
          FloatingActionButton.small(
            tooltip: 'Add column',
            onPressed: () {
              int columns = settings.get('boardColumnNum') ?? 2;
              if (columns < 5) {
                settings.put('boardColumnNum', columns + 1);
                setState();
              }
            },
            child: const Icon(Icons.add),
          ),
          const SizedBox(width: 3),
          FloatingActionButton.small(
            tooltip: 'Remove column',
            onPressed: () {
              int columns = settings.get('boardColumnNum') ?? 2;
              if (columns > 1) {
                settings.put('boardColumnNum', columns - 1);
                setState();
              }
            },
            child: const Icon(Icons.remove),
          ),
          const SizedBox(width: 20),
          FloatingActionButton(
            tooltip: 'Refresh',
            onPressed: () {
              ReqViewBoard().sendSignalToRust();
            },
            child: const Icon(Icons.refresh),
          ),
          const SizedBox(width: 20),
          FloatingActionButton(
            tooltip: 'Dump',
            onPressed: () => showDialog(
                context: context,
                builder: (context) => AlertDialog(
                        title: const Text('Dump'),
                        content: Column(
                          mainAxisSize: MainAxisSize.min,
                          children: [
                            const Text(
                                'Choose a name of the archive for the bulletin.'),
                            TextField(
                              decoration: const InputDecoration(
                                  hintText: 'Archive name'),
                              controller: archiveName,
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
                                  ReqDump(
                                    acvName: archiveName.text,
                                  ).sendSignalToRust();
                                  Navigator.pop(context);
                                  sleep(const Duration(milliseconds: 10));
                                  ReqViewBoard().sendSignalToRust();
                                }
                              },
                              child: const Text('Ok')),
                        ])),
            child: const Icon(Icons.archive_outlined),
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
                                  Navigator.pop(context);
                                  sleep(const Duration(milliseconds: 10));
                                  ReqViewBoard().sendSignalToRust();
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
