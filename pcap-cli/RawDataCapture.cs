using System;
using System.Collections.Generic;
using Chorizite.ACProtocol;
using Chorizite.ACProtocol.Messages;

namespace PcapCli
{
    /// <summary>
    /// Captures raw message bytes from PacketReader
    /// </summary>
    public class RawDataCapture
    {
        private readonly PacketReader _packetReader;
        private Queue<byte[]> _pendingRawData = new();
        private Dictionary<int, byte[]> _messageRawDataById = new();

        public RawDataCapture(PacketReader packetReader)
        {
            _packetReader = packetReader;
        }

        public void CaptureRawDataForMessage(int messageId)
        {
            // Get raw data from packet reader's queue
            var rawData = _packetReader.GetCompletedFragmentData();
            if (rawData != null && rawData.Length > 0)
            {
                _messageRawDataById[messageId] = rawData;
            }
        }

        public byte[]? GetRawDataForMessage(int messageId)
        {
            if (_messageRawDataById.TryGetValue(messageId, out var data))
            {
                return data;
            }
            return null;
        }
    }
}
