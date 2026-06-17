// See https://aka.ms/new-console-template for more information

using System;
using System.Runtime.InteropServices;

namespace wasiconsole;

public static class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine($"Hello World! {1} + {2} = {1 + 2}");
        Console.WriteLine($"Hello {RuntimeInformation.OSDescription}:{RuntimeInformation.OSArchitecture}");
        Console.WriteLine($"With love from {RuntimeInformation.FrameworkDescription}");
    }
}

/*
public static class MyWasmPipeline
{
    public static void Main(string[] args)
    {
        Console.WriteLine("Hello, World, ITS WASMI WASI main TIME AGAIN LETS GOO!!!!??");
        Console.WriteLine($"Hello {RuntimeInformation.OSDescription}:{RuntimeInformation.OSArchitecture}");
        Console.WriteLine($"With love from {RuntimeInformation.FrameworkDescription}");
        while (true)
        {
            var timeElapsed = SleepUntilNextUpdate();
            Console.WriteLine($"[C#]: Update, time elapsed: {timeElapsed}");
            Console.WriteLine($"[C#]: Mouse position: {GetMousePositionX()}");
        }
    }

    public static int GetMousePositionX()
    {
        return CallLiquislimeApi("get_mouse_position");
    }

    public static int CallLiquislimeApi(string method, params object[] args)
    {
        var path = $"./ls_api-{method}/{string.Join(",", args)}";
        return RawCall(path);
    }

    public static double SleepUntilNextUpdate()
    {
        var path = $"./ls_flow-sleep_until_next_update";
        Console.WriteLine($"Trying to open {path}");
        var milliseconds = RawCall(path);
        return milliseconds / 1000.0;
    }

    public static int RawCall(string fakeFile)
    {
        try
        {
            return (int)File.GetAttributes(fakeFile);
        }
        catch (FileNotFoundException)
        {
            // Console.WriteLine($"[C#]: File exception: {ex}");
            int customReturn = System.Runtime.InteropServices.Marshal.GetLastWin32Error();
            // return ex.HResult;
            return customReturn;
        }
        catch (DirectoryNotFoundException)
        {
            // Console.WriteLine($"[C#]: Dir exception: {ex}");
            int customReturn = System.Runtime.InteropServices.Marshal.GetLastWin32Error();
            // return ex.HResult;
            return customReturn;
        }
        catch (Exception ex)
        {
            Console.WriteLine($"[C#]: Unknown exception: {ex}");
            return ex.HResult;
        }
    }

}
*/