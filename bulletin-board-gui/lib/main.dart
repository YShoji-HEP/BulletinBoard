import 'dart:io';
import 'package:flutter/material.dart';
import 'package:rinf/rinf.dart';
import 'package:hive/hive.dart';
import 'package:bulletin_board/messages/all.dart';
import 'package:bulletin_board/pages/start.dart';
import 'package:bulletin_board/pages/board.dart';
import 'package:bulletin_board/pages/archives.dart';
import 'package:bulletin_board/pages/server.dart';
import 'package:bulletin_board/pages/settings.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await initializeRust(assignRustSignal);
  final currentDir = Directory.current.path;
  Hive.init(currentDir);
  await Hive.openBox('settings');
  final settings = Hive.box('settings');
  ReqSetAddr(address: settings.get('serverAddress') ?? '127.0.0.1:7578')
      .sendSignalToRust();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Bulletin Board',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.amber),
        useMaterial3: true,
      ),
      home: const MyHomePage(),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({
    super.key,
  });

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  var _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, constraints) {
      Widget page;
      switch (_selectedIndex) {
        case 0:
          page = const StartPage();
          break;
        case 1:
          page = const BoardPage();
          break;
        case 2:
          page = const ArchivesPage();
          break;
        case 3:
          page = const ServerPage();
          break;
        case 4:
          page = const SettingsPage();
          break;
        default:
          throw UnimplementedError('Page is not ready for $_selectedIndex');
      }
      return Scaffold(
        body: Row(
          children: [
            NavigationRail(
              leading: ClipRRect(
                  borderRadius: BorderRadius.circular(5),
                  child: const Image(
                      image: AssetImage("assets/bulletin-board.WEBP"),
                      width: 50)),
              minExtendedWidth: 170,
              extended: constraints.maxWidth >= 800,
              destinations: const [
                NavigationRailDestination(
                    icon: Icon(Icons.start), label: Text('Start')),
                NavigationRailDestination(
                    icon: Icon(Icons.tab), label: Text('Board')),
                NavigationRailDestination(
                    icon: Icon(Icons.folder_zip_outlined),
                    label: Text('Archives')),
                NavigationRailDestination(
                    icon: Icon(Icons.computer), label: Text('Server')),
                NavigationRailDestination(
                    icon: Icon(Icons.settings), label: Text('Settings'))
              ],
              selectedIndex: _selectedIndex,
              onDestinationSelected: (value) {
                setState(() {
                  _selectedIndex = value;
                });
              },
            ),
            Expanded(
              child: Container(
                  color: Theme.of(context).colorScheme.inversePrimary,
                  child: page),
            ),
          ],
        ),
      );
    });
  }
}
