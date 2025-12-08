using System.Net;
using System.Reflection;
using Chorizite.ACProtocol;
using Chorizite.ACProtocol.Enums;
using Chorizite.ACProtocol.Messages;
using Medo.IO.Pcap;

if (args.Length != 1)
{
    Console.Error.WriteLine("Usage: dotnet run -- <pcap_file>");
    Environment.Exit(1);
}

string pcapFile = args[0];
Console.WriteLine($"Processing packet capture: {pcapFile}");

if (!File.Exists(pcapFile))
{
    Console.Error.WriteLine($"File not found: {pcapFile}");
    Environment.Exit(1);
}

string dataDir = Path.GetFullPath(Path.Combine("..", "data"));
if (!Directory.Exists(dataDir))
{
    Console.Error.WriteLine($"Our directory not found: {dataDir}");
    Environment.Exit(1);
}
List<ACMessage> messages = new List<ACMessage>();

// Read and parse the pcap file using Chorizite.ACProtocol
using (var fileStream = File.OpenRead(pcapFile))
using (var reader = new Medo.IO.Pcap.PcapReader(fileStream))
{
    var packetReader = new PacketReader(null, new MessageReader());
    var packetId = 0;
    var messageId = 0;

    packetReader.OnC2SPacket += (s, e) =>
    {
        Console.WriteLine($"C2S Packet {packetId}: {e.Packet.Header.Flags}");

        foreach (var message in e.Packet.Messages)
        {
            Console.WriteLine($"  Message {messageId}: {message.GetType().Name}");
            messageId++;
            messages.Add(message);
        }
        packetId++;
    };

    packetReader.OnS2CPacket += (s, e) =>
    {
        Console.WriteLine($"S2C Packet {packetId}: {e.Packet.Header.Flags}");

        foreach (var message in e.Packet.Messages)
        {
            Console.WriteLine($"  Message {messageId}: {message.GetType().Name}");
            messageId++;
            messages.Add(message);
        }
        packetId++;
    };

    var block = reader.NextBlock(true);
    while (block != null)
    {
        if (block is not PcapClassicPacket packet)
        {
            block = reader.NextBlock();
            continue;
        }

        var packetData = packet.GetData();
        if (packetData.Length < 14) continue;

        var etherType = (packetData[12] << 8) | packetData[13];
        if (etherType == 0x0800 && packetData.Length >= 34) // IPv4
        {
            var sourceIP = new IPAddress(packetData.Skip(26).Take(4).ToArray());
            var destIP = new IPAddress(packetData.Skip(30).Take(4).ToArray());

            var localIP = IPAddress.Parse("127.0.0.1");

            if (sourceIP.Equals(localIP))
            {
                packetReader.HandleC2SPacket(packetData.Skip(42).ToArray());
            }
            else if (destIP.Equals(localIP))
            {
                packetReader.HandleS2CPacket(packetData.Skip(42).ToArray());
            }
        }

        block = reader.NextBlock();
    }


}

Console.WriteLine("PCAP processing complete.");
Console.WriteLine($"Got {messages.Count} messages");

var idx = 0;
foreach (var message in messages)
{
    var dir = message.MessageDirection.ToString();
    string name = message.GetType().Name;
    var file_name = $"{idx:0000}_{dir}_{message.OpCode}_{name}.bin";
    string out_path = Path.Combine(dataDir, file_name);

    Console.WriteLine($"Writing {out_path}...");
    using (var stream = new FileStream(out_path, FileMode.Create, FileAccess.Write))
    using (var writer = new BinaryWriter(stream))
    {
        message.Write(writer);
    }
    Console.WriteLine($"Done {out_path}...");

    idx = idx + 1;
}
