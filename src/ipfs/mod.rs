pub fn add(x: &str) -> i32 {
    3
}

enum Cmd {
    Add,
    Bitswap(BitswapCmd),
    Block(BlockCmd),
    Bootstrap(BootstrapCmd),
    Cat,
    Commands,
    Config(ConfigCmd),
    Dag(DagCmd),
    Dht(DhtCmd),
    Diag(DiagCmd),
    Dns,
    File(FileCmd),
    Files(FilesCmd),
    Filestore(FilestoreCmd),
    Get,
    Id,
    Key(KeyCmd),
    Log(LogCmd),
    Ls,
    Mount,
    Name(NameCmd),
    Object(ObjectCmd),
    P2p(P2pCmd),
    Pin(PinCmd),
    Ping,
    Pubsub(PubsubCmd),
    Refs(RefsCmd),
    Repo(RepoCmd),
    Resolve,
    Shutdown,
    Stats(StatsCmd),
    Swarm(SwarmCmd),
    Tar(TarCmd),
    Update,
    Version,
}

enum BitswapCmd {
    Ledger,
    Reprovide,
    Stat,
    Unwant,
    Wantlist,
}

enum BlockCmd {
    Get,
    Put,
    Rm,
    Stat,
}

enum BootstrapCmd {
    AddDefault,
    List,
    RmAll,
}

enum ConfigCmd {
    Edit,
    Replace,
    Show,
}
enum DagCmd {
    Get,
    Put,
    Resolve,
}
enum DhtCmd {
    Findpeer,
    Findprovs,
    Get,
    Provide,
    Put,
    Query,
}
enum DiagCmd {
    CmdsClear,
    CmdsSetTime,
    Sys,
}


enum FileCmd {
    Ls,
}
enum FilesCmd {
    Cp,
    Flush,
    Ls,
    Mkdir,
    Mv,
    Read,
    Rm,
    Stat,
    Write,
}
enum FilestoreCmd {
    Dups,
    Ls,
    Verify,
}


enum KeyCmd {
    Gen,
    List,
    Rename,
    Rm,
}
enum LogCmd {
    Level,
    Ls,
    Tail,
}


enum NameCmd {
    Publish,
    Resolve,
}
enum ObjectCmd {
    Data,
    Diff,
    Get,
    Links,
    New,
    PatchAddLink,
    PatchAppendData,
    PatchRmLink,
    PatchSetData,
    Put,
    Stat,
}
enum P2pCmd {
    ListenerClose,
    ListenerLs,
    ListenerOpen,
    StreamClose,
    StreamDial,
    StreamLs,
}
enum PinCmd {
    Add,
    Ls,
    Rm,
    Update,
    Verify,
}


enum PubsubCmd {
    Ls,
    Peers,
    Pub,
    Sub,
}
enum RefsCmd {
    Local,
}
enum RepoCmd {
    Fsck,
    Gc,
    Stat,
    Verify,
    Version,
}


enum StatsCmd {
    Bw,
    Repo,
}

enum SwarmCmd {
    AddrsListen,
    AddrsLocal,
    Connect,
    Disconnect,
    FiltersAdd,
    FiltersRm,
    Peers,
}
enum TarCmd {
    Add,
    Cat,
}
