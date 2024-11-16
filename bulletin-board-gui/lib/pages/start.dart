import 'package:hive/hive.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:bulletin_board/messages/all.dart';
import 'package:bulletin_board/common/enums.dart';

class StartPage extends StatelessWidget {
  const StartPage({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    final settings = Hive.box('settings');
    bool builtInServer = settings.get('builtInServer') ?? false;
    final targetLanguage =
        TargetLanguage.values[settings.get('targetLanguage') ?? 0];
    final pythonAlias = settings.get('pythonAlias') ?? '';
    final serverAddress = settings.get('serverAddress') ?? '127.0.0.1:7578';
    final listenAddress = settings.get('listenAddress') ?? '127.0.0.1:7578';

    final Uri githubSponsors =
        Uri.parse('https://github.com/sponsors/YShoji-HEP');
    final Uri buyMeaCoffee = Uri.parse('https://buymeacoffee.com/yshojihep');

    String message;
    String snippet;

    switch (targetLanguage) {
      case TargetLanguage.mathematica:
        {
          message =
              'To start, click and copy the following, and paste it to a Mathematica notebook';
          snippet =
              '<< "Yshojihep`BulletinBoardClient`";\nBBSetAddr["$serverAddress"];';
        }
      case TargetLanguage.python:
        {
          message =
              'To start, click and copy the following, and paste it to a Jupyter notebook';
          if (pythonAlias == '') {
            snippet =
                'import bulletin_board_client\nbulletin_board_client.set_addr("$serverAddress")';
          } else {
            snippet =
                'import bulletin_board_client as $pythonAlias\n$pythonAlias.set_addr("$serverAddress")';
          }
        }
    }

    final titleStyle = Theme.of(context)
        .textTheme
        .headlineLarge!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);

    final largerStyle = Theme.of(context)
        .textTheme
        .bodyLarge!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);

    final bodyStyle = Theme.of(context)
        .textTheme
        .bodyMedium!
        .copyWith(color: Theme.of(context).colorScheme.onPrimaryContainer);

    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        Text(
          'Welcome!',
          style: titleStyle,
        ),
        const SizedBox(
          height: 30,
        ),
        Visibility(
          visible: builtInServer,
          maintainState: true,
          maintainAnimation: true,
          child: Column(
            children: [
              Text(
                'Built-in Server',
                style: largerStyle,
              ),
              const SizedBox(
                height: 10,
              ),
              Row(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Tooltip(
                      message: "Start",
                      child: OutlinedButton(
                        onPressed: () {
                          ReqStartServer(address: listenAddress)
                              .sendSignalToRust();
                        },
                        child: const Icon(Icons.play_arrow),
                      )),
                  const SizedBox(
                    width: 20,
                  ),
                  Tooltip(
                      message: "Stop",
                      child: OutlinedButton(
                        onPressed: () {
                          ReqStopServer().sendSignalToRust();
                        },
                        child: const Icon(Icons.stop),
                      )),
                ],
              ),
              const SizedBox(
                height: 30,
              ),
            ],
          ),
        ),
        Text(
          message,
          style: bodyStyle,
        ),
        const SizedBox(
          height: 10,
        ),
        Material(
          elevation: 1,
          borderRadius: BorderRadius.circular(5),
          color: Theme.of(context).colorScheme.primaryContainer,
          child: InkWell(
            borderRadius: BorderRadius.circular(5),
            onTap: () async {
              await Clipboard.setData(ClipboardData(text: snippet));
            },
            child: Padding(
              padding: const EdgeInsets.all(7.0),
              child: Text(snippet),
            ),
          ),
        ),
        const SizedBox(
          height: 30,
        ),
        Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            IconButton(
              tooltip: "GitHub Sponsors",
              icon: const Icon(Icons.favorite),
              onPressed: () async {
                await launchUrl(githubSponsors);
              },
            ),
            const SizedBox(
              width: 5,
            ),
            IconButton(
              tooltip: "Buy me a coffee",
              icon: const Icon(Icons.coffee),
              onPressed: () async {
                await launchUrl(buyMeaCoffee);
              },
            ),
          ],
        ),
      ],
    );
  }
}
