namespace LibraryWorld.wit.Exports.example.component;

public class OperationsExportsImpl : IOperationsExports
{
    public static int Add(int left, int right)
    {
        // wit.Imports.example.component.OperationsImportsInterop.GetMousePosition();
        return left + WasmImportAdd(right, 100);
    }

    [System.Runtime.InteropServices.DllImportAttribute("example:component/operations", EntryPoint = "add"), System.Runtime.InteropServices.WasmImportLinkageAttribute]
    public static extern int WasmImportAdd(int p0, int p1);

}
