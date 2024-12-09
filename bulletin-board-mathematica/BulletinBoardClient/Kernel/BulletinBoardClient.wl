(* ::Package:: *)

(* ::Section:: *)
(*Package Header*)


BeginPackage["Yshojihep`BulletinBoardClient`"];


BBBuildLibrary::usage = "BBBuildLibrary[] builds the client library.";
BBSetAddr::usage = "BBSetAddr[address] sets the address of the server.";
BBSetTimeout::usage = "BBSetTimeout[timeout] sets timeout for TCP connections.";
BBPost::usage = "BBPost[title, tag(optional), data] sends data to the server.";
BBRead::usage = "BBRead[title, tag(optional), revisions(optional)] retrives data from the server.";
BBRelabel::usage = "BBRelabel[titleFrom, tagFrom, titleTo, tagTo] relabels a bulletin.";
BBClientVersion::usage = "BBClientVersion[] returns the client version.";
BBServerVersion::usage = "BBServerVersion[] returns the server version.";
BBStatus::usage = "BBStatus[] returns the status of the server.";
BBLog::usage = "BBLog[] returns the log of the server.";
BBViewBoard::usage = "BBViewBoard[] returns the list of bulletins.";
BBGetInfo::usage = "BBGetInfo[title, tag(optional)] retrives the information of the bulletin.";
BBClearRevisions::usage = "BBClearRevisions[title, tag(optional), revisions] deletes specified revisions of the bulletin.";
BBRemove::usage = "BBRemove[title, tag(optional)] removes a bulletin.";
BBArchive::usage = "BBArchive[title, tag(optional), archiveName] saves a bulletin into an archive.";
BBLoad::usage = "BBLoad[archiveName] loads bulletins from an archive.";
BBListArchive::usage = "BBListArchive[] returns the list of archives.";
BBRenameArchive::usage = "BBRenameArchive[archiveFrom, archiveTo] renames an archive.";
BBDeleteArchive::usage = "BBDeleteArchive[archiveName] deletes an archive.";
BBDump::usage = "BBDump[archiveName] saves all the bulletins into an archive.";
BBRestore::usage = "BBRestore[archiveName] restores bulletins from an archive.";
BBClearLog::usage = "BBClearLog[archiveName] clears the log of the server.";
BBResetServer::usage = "BBResetServer[] resets the server.";
BBTerminateServer::usage = "BBTerminateServer[] terminates the server.";


Begin["`Private`"];


(* ::Section:: *)
(*Definitions*)


libraryDir=FileNameJoin[{ParentDirectory[DirectoryName[$InputFileName]],"LibraryResources"}];

libraryName=Switch[$OperatingSystem,"MacOSX","libbulletin_board_mathematica.dylib","Windows","bulletin_board_mathematica.dll",_,"libbulletin_board_mathematica.so"];

libraryPath=FileNameJoin[{libraryDir,libraryName}];


loadFuncitons:=Module[{lib,loader},
	lib=LibraryFunctionLoad[libraryPath,"load_dbgbb",LinkObject,LinkObject];
	loader=lib[libraryPath];
	BBSetAddr=loader["set_addr"];
	BBSetTimeout=loader["set_timeout"];
	BBPostInteger=loader["post_integer"];
	BBPostReal=loader["post_real"];
	BBPostComplex=loader["post_complex"];
	BBPostString=loader["post_string"];
	BBPostIntegerArray=loader["post_integer_array"];
	BBPostRealArray=loader["post_real_array"];
	BBPostComplexArray=loader["post_complex_array"];
	BBPostStringArray=loader["post_string_array"];
	BBRead=loader["read"];
	BBRelabel=loader["relabel"];
	BBClientVersion=loader["client_version"];
	BBServerVersion=loader["server_version"];
	BBStatusRaw=loader["status"];
	BBLog=loader["log"];
	BBViewBoardRaw=loader["view_board"];
	BBGetInfoRaw=loader["get_info"];
	BBClearRevisions=loader["clear_revisions"];
	BBRemove=loader["remove"];
	BBArchive=loader["archive"];
	BBLoad=loader["load"];
	BBListArchive=loader["list_archive"];
	BBRenameArchive=loader["rename_archive"];
	BBDeleteArchive=loader["delete_archive"];
	BBDump=loader["dump"];
	BBRestore=loader["restore"];
	BBClearLog=loader["clear_log"];
	BBResetServer=loader["reset_server"];
	BBTerminateServer=loader["terminate_server"];
]

CADirSuggest=FileNameJoin[{$InstallationDirectory, "SystemFiles", "Links", "WSTP", 
  "DeveloperKit", $SystemID, "CompilerAdditions"}];

If[FileExistsQ[libraryPath],
	loadFuncitons;
	Print["BulletinBoardClient loaded. Client version: "<>BBClientVersion[]];,
	Print["The BulletinBoardClient library does not exist. To compile it, please follow the instruction below.

1. A standard C++ build environment is required. Install it following instructions available on the Internet.
2. If you have already installed a Rust toolchain and the cargo-clone crate, run BBBuildLibrary[\"CompilerAdditionsDirectory\"->\"dir/to/CompilerAdditions\"].
   If you do not and would prefer not to install Rust in your environment, run BBBuildLibrary[\"DownloadRust\"->True,\"CompilerAdditionsDirectory\"->\"dir/to/CompilerAdditions\"].
   Then, the toolchain will be downloaded in the Paclet directory.
3. If it does not compile, refer to the log and install any missing libraries.

* You can manually download the source from crates.io or GitHub, and build the library. Then, copy the compiled library to "<>libraryDir<>".

* The wolfram-app-discovery crate often fails to find the WSTP CompilerAddtions directory, which is typically located at \""<>CADirSuggest<>"\". The directory can be set by
BBBuildLibrary[\"CompilerAdditionsDirectory\"->\"dir/to/CompilerAdditions\"]

* The version of client can be set by
BBBuildLibrary[\"ClientVersion\"->\"0.3.2\"]
Notice that this paclet version is compatible with 0.3.2+."
	];
]


BBBuildLibrary[opt___]:=If[$OperatingSystem=="Windows",BBBuildLibraryWindows[opt],BBBuildLibraryUnix[opt]]


Options[BBBuildLibraryUnix]={"CompilerAdditionsDirectory"->None,"ClientVersion"->None,"DownloadRust"->False};

BBBuildLibraryUnix[OptionsPattern[]]:=Module[{prolog,cargo,installRustCommand,installCloneCommand,cloneClientCommand,buildClientCommand,compilerAdditionsDirectory=OptionValue["CompilerAdditionsDirectory"],ClientVersion=OptionValue["ClientVersion"],DownloadRust=OptionValue["DownloadRust"]},
	prolog=If[DownloadRust,"export RUSTUP_HOME="<>libraryDir<>"/rustup CARGO_HOME="<>libraryDir<>"/cargo ;",""]<>If[compilerAdditionsDirectory===None,"","export WSTP_COMPILER_ADDITIONS_DIRECTORY=\""<>compilerAdditionsDirectory<>"\" "];
	cargo=If[DownloadRust,libraryDir<>"/cargo/bin/cargo ","~/.cargo/bin/cargo "];

	If[!DirectoryQ[libraryDir],CreateDirectory[libraryDir]];

	If[DownloadRust,
		installRustCommand="curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --no-modify-path";
		Print[installRustCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog},installRustCommand]];
		installCloneCommand=libraryDir<>"/cargo/bin/cargo install cargo-clone";
		Print[installCloneCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog},installCloneCommand]];
	];

	cloneClientCommand=cargo<>"clone bulletin-board-mathematica"<>If[ClientVersion===None,"","@"<>ClientVersion]<>" -- "<>libraryDir<>"/bulletin-board-mathematica";
	Print[cloneClientCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog},cloneClientCommand]];

	buildClientCommand=cargo<>"build -r --manifest-path "<>libraryDir<>"/bulletin-board-mathematica/Cargo.toml";
	Print[buildClientCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog},buildClientCommand]];

	CopyFile[FileNameJoin[{libraryDir,"bulletin-board-mathematica/target/release",libraryName}],libraryPath];

	DeleteDirectory[libraryDir<>"/bulletin-board-mathematica",DeleteContents->True];

	loadFuncitons
]


Options[BBBuildLibraryWindows]={"CompilerAdditionsDirectory"->None,"ClientVersion"->None,"DownloadRust"->False};

BBBuildLibraryWindows[OptionsPattern[]]:=Module[{prolog,cargo,powerShell,installRustCommand,installCloneCommand,cloneClientCommand,buildClientCommand,CompilerAdditionsDirectory=OptionValue["CompilerAdditionsDirectory"],ClientVersion=OptionValue["ClientVersion"],DownloadRust=OptionValue["DownloadRust"]},
	powerShell=FileNameJoin[{Environment["SystemRoot"], "system32", 
     "WindowsPowerShell", "v1.0", "powershell.exe"}];
	prolog=If[DownloadRust,"$Env:RUSTUP_HOME=\""<>libraryDir<>"\\rustup\"; $Env:CARGO_HOME=\""<>libraryDir<>"\\cargo\"; ",""]<>If[CompilerAdditionsDirectory===None,"","$Env:WSTP_COMPILER_ADDITIONS_DIRECTORY=\""<>CompilerAdditionsDirectory<>"\"; "];
	cargo=If[DownloadRust,libraryDir<>"\\cargo\\bin\\cargo ","~\\.cargo\\bin\\cargo "];

	If[!DirectoryQ[libraryDir],CreateDirectory[libraryDir]];

	If[DownloadRust,
		URLDownload["https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe",FileNameJoin[{libraryDir,"rustup-init.exe"}]];
		installRustCommand=libraryDir<>"\\rustup-init.exe -y --profile minimal --no-modify-path";
		Print[installRustCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog,"Evaluator"->powerShell},installRustCommand]];
		installCloneCommand=libraryDir<>"\\cargo\\bin\\cargo install cargo-clone";
		Print[installCloneCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog,"Evaluator"->powerShell},installCloneCommand]];
	];

	cloneClientCommand=cargo<>"clone bulletin-board-mathematica"<>If[ClientVersion===None,"","@"<>ClientVersion]<>" -- "<>libraryDir<>"\\bulletin-board-mathematica";
	Print[cloneClientCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog,"Evaluator"->powerShell},cloneClientCommand]];

	buildClientCommand=cargo<>"build -r --manifest-path "<>libraryDir<>"\\bulletin-board-mathematica\\Cargo.toml";
	Print[buildClientCommand];Print[ExternalEvaluate[{"Shell","SessionProlog"->prolog,"Evaluator"->powerShell},buildClientCommand]];

	CopyFile[FileNameJoin[{libraryDir,"bulletin-board-mathematica\\target\\release",libraryName}],libraryPath];

	DeleteDirectory[libraryDir<>"\\bulletin-board-mathematica",DeleteContents->True];

	loadFuncitons
]


BBViewBoard[]:=Enclose[Module[{result=Confirm[BBViewBoardRaw[]]},
<|"title"->#[[1]],"tag"->#[[2]],"revisions"->#[[3]]|>&/@result]]


BBGetInfo[input__]:=Enclose[Module[{result=Confirm[BBGetInfoRaw[input]]},
<|"revision"->#[[1]],"datasize"->#[[2]],"timestamp"->#[[3]],"backend"->#[[4]]|>&/@result]]


BBStatus[]:=Enclose[Module[{result=Confirm[BBStatusRaw[]]},
<|"datasize"->result[[1]],"memory_used"->result[[2]],"memory_used(%)"->result[[3]],"objects"->result[[4]],"files"->result[[5]],"archived"->result[[6]]|>]]


BBPost[title_,tag_,data_]:=Enclose[Switch[Head[data],
	Integer,BBPostInteger[title,tag,data],
	Real,BBPostReal[title,tag,data],
	Complex,BBPostComplex[title,tag,Re[data],Im[data]],
	String,BBPostString[title,tag,data],
	List,ConfirmAssert[ArrayQ[data]&&Length[DeleteDuplicates[Head/@Flatten[data]]]==1];Switch[Head[Flatten[data][[1]]],
		Integer,BBPostIntegerArray[title,tag,data],
		Real,BBPostRealArray[title,tag,data],
		Complex,BBPostComplexArray[title,tag,Re[data],Im[data]],
		String,BBPostStringArray[title,tag,Flatten[data],Dimensions[data]],
		_,Throw["Wrong datatype."]
	],
	_,Throw["Wrong datatype."]
]]


BBPost[title_,data_]:=BBPost[title,"Mathematica",data]


(* ::Section:: *)
(*Package Footer*)


End[];
EndPackage[];
