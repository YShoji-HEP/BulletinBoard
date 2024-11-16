enum SortingField {
  title("Title"),
  tag("Tag");

  const SortingField(this.label);
  final String label;
}

enum SortingOrder {
  ascending,
  descending,
}

enum BoardClickAction { clipboard, palette }

enum TargetLanguage {
  mathematica('Mathematica'),
  python('Python');
  // text1('Plain Text [...]'),
  // text2('Plain Text {...}')

  const TargetLanguage(this.label);
  final String label;
}
