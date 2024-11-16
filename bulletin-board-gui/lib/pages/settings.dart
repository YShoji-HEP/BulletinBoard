import 'package:flutter/material.dart';
import 'package:hive/hive.dart';
import 'package:bulletin_board/messages/all.dart';
import 'package:bulletin_board/common/enums.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({
    super.key,
  });

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  @override
  Widget build(BuildContext context) {
    final settings = Hive.box('settings');
    bool builtInServer = settings.get('builtInServer') ?? false;
    bool copyFull = settings.get('copyFull') ?? false;
    final targetLanguage =
        TargetLanguage.values[settings.get('targetLanguage') ?? 0];
    final clickAction =
        BoardClickAction.values[settings.get('boardClickAction') ?? 0];
    final pythonAlias =
        TextEditingController(text: settings.get('pythonAlias') ?? '');
    final serverAddress =
        TextEditingController(text: settings.get('serverAddress') ?? '');
    final listenAddress =
        TextEditingController(text: settings.get('listenAddress') ?? '');

    final needsAlias = targetLanguage == TargetLanguage.python;

    final indexStyle = Theme.of(context)
        .textTheme
        .headlineMedium!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);
    final sectionStyle = Theme.of(context)
        .textTheme
        .bodyLarge!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);

    return Padding(
      padding: const EdgeInsets.all(20.0),
      child: ListView(
        children: [
          Text('Server settings', style: indexStyle),
          RadioListTile<bool>(
            title: const Text('Built-in server'),
            subtitle: const Text('Use the built-in Bulletin Board server.'),
            value: true,
            groupValue: builtInServer,
            onChanged: (value) => setState(() {
              String listenAddress =
                  settings.get('listenAddress') ?? '127.0.0.1:7578';
              RegExp exp = RegExp(r':[0-9]+$');
              final match = exp.firstMatch(listenAddress);
              String port;
              if (match == null) {
                port = ':7578';
              } else {
                port = match[0]!;
              }
              final serverAddress = '127.0.0.1$port';
              ReqSetAddr(address: serverAddress).sendSignalToRust();
              settings.put('builtInServer', true);
            }),
          ),
          RadioListTile<bool>(
            title: const Text('Remote server'),
            subtitle: const Text('Connect to a remote Bulletin Board server.'),
            value: false,
            groupValue: builtInServer,
            onChanged: (value) => setState(() {
              ReqSetAddr(
                      address:
                          settings.get('serverAddress') ?? '127.0.0.1:7578')
                  .sendSignalToRust();
              settings.put('builtInServer', false);
            }),
          ),
          const SizedBox(
            height: 5,
          ),
          const SizedBox(
            height: 5,
          ),
          Visibility(
            visible: builtInServer,
            maintainState: true,
            maintainAnimation: true,
            child: Column(
              children: [
                AnimatedOpacity(
                  duration: const Duration(milliseconds: 500),
                  curve: Curves.fastOutSlowIn,
                  opacity: builtInServer ? 1 : 0,
                  child: Column(
                    children: [
                      TextField(
                        controller: listenAddress,
                        decoration: const InputDecoration(
                            labelText: 'Listen address',
                            hintText: '127.0.0.1:7578 or /path/to/sock'),
                      ),
                      const SizedBox(
                        height: 10,
                      ),
                      Row(children: [
                        Tooltip(
                            message: 'Apply',
                            child: OutlinedButton(
                                onPressed: () {
                                  RegExp exp = RegExp(r':[0-9]+$');
                                  final match =
                                      exp.firstMatch(listenAddress.text);
                                  String port;
                                  if (match == null) {
                                    port = ':7578';
                                  } else {
                                    port = match[0]!;
                                  }
                                  final serverAddress = '127.0.0.1$port';
                                  ReqSetAddr(address: serverAddress)
                                      .sendSignalToRust();
                                  settings.put(
                                      'listenAddress', listenAddress.text);
                                  settings.put('serverAddress', serverAddress);
                                },
                                child: const Icon(Icons.check)))
                      ]),
                    ],
                  ),
                ),
                const SizedBox(
                  height: 5,
                ),
              ],
            ),
          ),
          Visibility(
            visible: !builtInServer,
            maintainState: true,
            maintainAnimation: true,
            child: Column(
              children: [
                AnimatedOpacity(
                  duration: const Duration(milliseconds: 500),
                  curve: Curves.fastOutSlowIn,
                  opacity: builtInServer ? 0 : 1,
                  child: Column(
                    children: [
                      TextField(
                        controller: serverAddress,
                        decoration: const InputDecoration(
                            labelText: 'Server address',
                            hintText: '127.0.0.1:7578 or /path/to/sock'),
                      ),
                      const SizedBox(
                        height: 5,
                      ),
                      Row(children: [
                        Tooltip(
                            message: 'Apply',
                            child: OutlinedButton(
                                onPressed: () {
                                  ReqSetAddr(address: serverAddress.text)
                                      .sendSignalToRust();
                                  settings.put(
                                      'serverAddress', serverAddress.text);
                                },
                                child: const Icon(Icons.check)))
                      ]),
                    ],
                  ),
                ),
              ],
            ),
          ),
          const SizedBox(
            height: 10,
          ),
          Text('Snippet settings', style: indexStyle),
          const SizedBox(
            height: 20,
          ),
          DropdownMenu<TargetLanguage>(
            initialSelection: targetLanguage,
            label: const Text('Notebook Language'),
            onSelected: (lang) => {
              setState(() {
                settings.put('targetLanguage', lang?.index);
              })
            },
            dropdownMenuEntries: TargetLanguage.values
                .map((val) => DropdownMenuEntry<TargetLanguage>(
                    value: val, label: val.label))
                .toList(),
          ),
          const SizedBox(
            height: 10,
          ),
          Visibility(
            visible: needsAlias,
            maintainState: true,
            maintainAnimation: true,
            child: Column(
              children: [
                AnimatedOpacity(
                  duration: const Duration(milliseconds: 500),
                  curve: Curves.fastOutSlowIn,
                  opacity: needsAlias ? 1 : 0,
                  child: TextField(
                      controller: pythonAlias,
                      decoration: const InputDecoration(
                          labelText: 'Alias', hintText: 'bbclient'),
                      onChanged: (text) {
                        settings.put('pythonAlias', text);
                      }),
                ),
              ],
            ),
          ),
          const SizedBox(
            height: 10,
          ),
          Text('Click action:', style: sectionStyle),
          RadioListTile<BoardClickAction>(
            title: const Text('Clipboard'),
            subtitle: const Text('Copy to clipboard.'),
            value: BoardClickAction.clipboard,
            groupValue: clickAction,
            onChanged: (value) => {
              setState(() {
                settings.put('boardClickAction', value?.index);
              })
            },
          ),
          RadioListTile<BoardClickAction>(
            title: const Text('Palette (experimental)'),
            subtitle: const Text('Directly input via a simulated keyboard.'),
            value: BoardClickAction.palette,
            groupValue: clickAction,
            onChanged: (value) => {
              setState(() {
                settings.put('boardClickAction', value?.index);
              })
            },
          ),
          Text('Copy mode:', style: sectionStyle),
          RadioListTile<bool>(
            title: const Text('Minimal expression'),
            subtitle: const Text('Omit tags if not necessary.'),
            value: false,
            groupValue: copyFull,
            onChanged: (_) => {
              setState(() {
                settings.put('copyFull', false);
              })
            },
          ),
          RadioListTile<bool>(
            title: const Text('Full expression'),
            subtitle: const Text('Always copy with tags.'),
            value: true,
            groupValue: copyFull,
            onChanged: (_) => {
              setState(() {
                settings.put('copyFull', true);
              })
            },
          ),
        ],
      ),
    );
  }
}
