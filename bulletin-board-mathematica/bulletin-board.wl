(* ::Package:: *)

BeginPackage["BulletinBoardClient`"];


BBLoadFunctions::usage = "BBLoadFunctions[address]";
BBPost::usage = "BBPost[varName, varTag(optional), data]";
BBRead::usage = "BBRead[varName, varTag(optional), revisions(optional)]";
BBStatus::usage = "BBStatus[]";
BBLog::usage = "BBLog[]";
BBViewBoard::usage = "BBViewBoard[]";
BBGetInfo::usage = "BBGetInfo[varName, varTag(optional)]";
BBClearRevisions::usage = "BBClearRevisions[varName, varTag, revisions]";
BBRemove::usage = "BBRemove[varName, varTag]";
BBArchive::usage = "BBArchive[varName, varTag, archive]";
BBLoad::usage = "BBLoad[archiveName]";
BBListArchive::usage = "BBListArchive[]";
BBRenameArchive::usage = "BBRenameArchive[archiveFrom, archiveTo]";
BBDeleteArchive::usage = "BBDeleteArchive[archiveName]";
BBDump::usage = "BBDump[archiveName]";
BBRestore::usage = "BBRestore[archiveName]";
BBReset::usage = "BBReset[]";


Begin["`Private`"];


path=FileNameJoin[DirectoryName[$InputFileName],"libbulletin_board_mathematica.dylib"];


BBLoadFunctions[address_]:=Module[{lib,loader},
	SetEnvironment["BB_ADDR"->address];
	lib=LibraryFunctionLoad[path,"load_dbgbb",LinkObject,LinkObject];
	loader=lib[path];
	BBPostInteger=loader["post_integer"];
	BBPostReal=loader["post_real"];
	BBPostComplex=loader["post_complex"];
	BBPostString=loader["post_string"];
	BBPostIntegerArray=loader["post_integer_array"];
	BBPostRealArray=loader["post_real_array"];
	BBPostComplexArray=loader["post_complex_array"];
	BBPostStringArray=loader["post_string_array"];
	BBRead=loader["read"];
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
	BBReset=loader["reset"];
]


BBViewBoard[]:=Enclose[Module[{result=Confirm[BBViewBoardRaw[]]},
<|"varName"->#[[1]],"varTag"->#[[2]],"revisions"->#[[3]]|>&/@result]]


BBGetInfo[input__]:=Enclose[Module[{result=Confirm[BBGetInfoRaw[input]]},
<|"revision"->#[[1]],"datasize"->#[[2]],"timestamp"->#[[3]],"backend"->#[[4]]|>&/@result]]


BBStatus[]:=Enclose[Module[{result=Confirm[BBStatusRaw[]]},
<|"datasize"->result[[1]],"memory_used"->result[[2]],"memory_used(%)"->result[[3]],"bulletins"->result[[4]],"files"->result[[5]],"archived"->result[[6]]|>]]


BBPost[varName_,varTag_,data_]:=Enclose[Switch[Head[data],
	Integer,BBPostInteger[varName,varTag,data],
	Real,BBPostReal[varName,varTag,data],
	Complex,BBPostComplex[varName,varTag,Re[data],Im[data]],
	String,BBPostString[varName,varTag,data],
	List,ConfirmAssert[ArrayQ[data]&&Length[DeleteDuplicates[Head/@Flatten[data]]]==1];Switch[Head[Flatten[data][[1]]],
		Integer,BBPostIntegerArray[varName,varTag,data],
		Real,BBPostRealArray[varName,varTag,data],
		Complex,BBPostComplexArray[varName,varTag,Re[data],Im[data]],
		String,BBPostStringArray[varName,varTag,Flatten[data],Dimensions[data]],
		_,Throw["Wrong datatype."]
	],
	_,Throw["Wrong datatype."]
]]


BBPost[varName_,data_]:=BBPost[varName,"Mathematica",data]


End[];


EndPackage[];
