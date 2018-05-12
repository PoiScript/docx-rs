using System;
using System.IO;
using DocumentFormat.OpenXml.Packaging;
using DocumentFormat.OpenXml.Validation;
using DocumentFormat.OpenXml.Wordprocessing;

namespace validator
{
  class Program
  {
    static int Main(string[] args)
    {
      if (args.Length == 0)
      {
        System.Console.WriteLine("Please enter path docx file.");
        System.Console.WriteLine("Usage: dotnet run [path]");
        return 128;
      }

      string filepath = Path.GetFullPath(args[0]);
      Console.WriteLine("Validating {0} ...", filepath);

      return ValidateWordDocument(filepath);
    }

    public static int ValidateWordDocument(string filepath)
    {
      try
      {
        WordprocessingDocument doc = WordprocessingDocument.Open(filepath, false);
        OpenXmlValidator validator = new OpenXmlValidator();
        int count = 0;
        foreach (ValidationErrorInfo error in validator.Validate(doc))
        {
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

        return 0;
      }

      catch (Exception ex)
      {
        Console.WriteLine(ex);

        return 1;
      }
    }
  }
}
