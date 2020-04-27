using System;
using System.IO;
using DocumentFormat.OpenXml.Packaging;
using DocumentFormat.OpenXml.Validation;
using DocumentFormat.OpenXml.Wordprocessing;

class Validator {
	static void Main(string[] args) {
		foreach(string filepath in Directory.GetFiles(Directory.GetCurrentDirectory(), "*.docx")) {
			ValidateWordDocument(filepath);
		}
	}

	public static void ValidateWordDocument(string filepath) {
		filepath = Path.GetFullPath(filepath);
		Console.WriteLine("Validating {0} ...", filepath);
		try {
			WordprocessingDocument doc = WordprocessingDocument.Open(filepath, false);
			OpenXmlValidator validator = new OpenXmlValidator();
			int count = 0;
			foreach(ValidationErrorInfo error in validator.Validate(doc)) {
				count++;
				Console.WriteLine("Error " + count);
				Console.WriteLine("Description: " + error.Description);
				Console.WriteLine("ErrorType: " + error.ErrorType);
				Console.WriteLine("Node: " + error.Node);
				Console.WriteLine("Path: " + error.Path.XPath);
				Console.WriteLine("Part: " + error.Part.Uri);
				Console.WriteLine("-------------------------------------------");
			}

			Console.WriteLine("count={0}", count);

			doc.Close();
		} catch(Exception e) {
			Console.WriteLine(e);
		}
	}
}
