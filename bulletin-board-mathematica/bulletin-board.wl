(* ::Package:: *)

BeginPackage["BulletinBoardClient`"];


BBSetAddr::usage = "BBSetAddr[address] sets the address of the server.";
BBPost::usage = "BBPost[title, tag(optional), data] sends data to the server.";
BBRead::usage = "BBRead[title, tag(optional), revisions(optional)] retrives data from the server.";
BBRelabel::usage = "BBRelabel[titleFrom, tagFrom, titleTo, tagTo] relabels a bulletin.";
BBVersion::usage = "BBVersion[] returns the version of the server.";
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


path=FileNameJoin[DirectoryName[$InputFileName],"libbulletin_board_mathematica.dylib"];

Module[{lib,loader},
	lib=LibraryFunctionLoad[path,"load_dbgbb",LinkObject,LinkObject];
	loader=lib[path];
	BBSetAddr=loader["set_addr"];
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
	BBVersion=loader["version"];
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


End[];


EndPackage[];
