// Full spell Id combining the spell id with the spell layer.
#[derive(Debug, Clone)]
pub struct LayeredSpellId {
    Id: String,
    Layer: String
}

// List which is packable for network
#[derive(Debug, Clone)]
pub struct PackableList {
    Count: String
}

// HashTable which is packable for network
#[derive(Debug, Clone)]
pub struct PackableHashTable {
    Count: String,
    MaxSize: String
}

// HashTable which is packable for network
#[derive(Debug, Clone)]
pub struct PHashTable {
    PackedSize: String
}

#[derive(Debug, Clone)]
pub struct Vector3 {
    X: String,
    Y: String,
    Z: String
}

#[derive(Debug, Clone)]
pub struct Quaternion {
    W: String,
    X: String,
    Y: String,
    Z: String
}

// Landcell location, without orientation
#[derive(Debug, Clone)]
pub struct Origin {
    Landcell: String,
    Location: String
}

// Landcell location, including orientation
#[derive(Debug, Clone)]
pub struct Position {
    Landcell: String,
    Frame: String
}

// A the location and orientation of an object within a landcell
#[derive(Debug, Clone)]
pub struct Frame {
    Origin: String,
    Orientation: String
}

// Optional header data when PacketHeaderFlags includes ServerSwitch
#[derive(Debug, Clone)]
pub struct ServerSwitchHeader {
    Sequence: String,
    Type: String
}

// Optional header data when PacketHeaderFlags includes CICMDCommand
#[derive(Debug, Clone)]
pub struct CICMDCommandHeader {
    Command: String,
    Parameter: String
}

// Optional header data when PacketHeaderFlags includes Flow
#[derive(Debug, Clone)]
pub struct FlowHeader {
    Bytes: String,
    Interval: String
}

// Optional header data when PacketHeaderFlags includes LogonServerAddr
#[derive(Debug, Clone)]
pub struct SocketAddress {
    Family: String,
    Port: String,
    Address: String,
    Empty: String
}

// Optional header data when PacketHeaderFlags includes LoginRequest
#[derive(Debug, Clone)]
pub struct LoginRequestHeader {
    ClientVersion: String,
    Length: String,
    AuthType: String,
    Flags: String,
    Sequence: String,
    Account: String,
    AccountToLoginAs: String,
    Password: String,
    GlsTicket: String
}

// Optional header data when PacketHeaderFlags includes Referral
#[derive(Debug, Clone)]
pub struct ReferralHeader {
    Cookie: String,
    Address: String,
    IdServer: String,
    Unknown: String
}

// Optional header data when PacketHeaderFlags includes ConnectRequest
#[derive(Debug, Clone)]
pub struct ConnectRequestHeader {
    ServerTime: String,
    Cookie: String,
    NetID: String,
    OutgoingSeed: String,
    IncomingSeed: String,
    Unknown: String
}

#[derive(Debug, Clone)]
pub struct NetError {
    StringId: String,
    TableId: String
}

#[derive(Debug, Clone)]
pub struct EchoResponseHeader {
    LocalTime: String,
    HoldingTime: String
}

// A collection of property tables.
#[derive(Debug, Clone)]
pub struct ACBaseQualities {
    Flags: String,
    WeenieType: String,
    IntProperties: String,
    Int64Properties: String,
    BoolProperties: String,
    FloatProperties: String,
    StringProperties: String,
    DataProperties: String,
    InstanceProperties: String,
    PositionProperties: String
}

// The ACQualities structure contains character property lists.
#[derive(Debug, Clone)]
pub struct ACQualities {
    Flags: String,
    HasHealth: String,
    Attributes: String,
    Skills: String,
    Body: String,
    SpellBook: String,
    Enchantments: String,
    EventFilter: String,
    Emotes: String,
    CreationProfile: String,
    PageData: String,
    Generators: String,
    GeneratorRegistry: String,
    GeneratorQueue: String
}

// The AttributeCache structure contains information about a character attributes.
#[derive(Debug, Clone)]
pub struct AttributeCache {
    Flags: String,
    Strength: String,
    Endurance: String,
    Quickness: String,
    Coordination: String,
    Focus: String,
    Self: String,
    Health: String,
    Stamina: String,
    Mana: String
}

// The Attribute structure contains information about a character attribute.
#[derive(Debug, Clone)]
pub struct AttributeInfo {
    PointsRaised: String,
    InnatePoints: String,
    ExperienceSpent: String
}

// The SecondaryAttribute structure contains information about a character vital.
#[derive(Debug, Clone)]
pub struct SecondaryAttributeInfo {
    Attribute: String,
    Current: String
}

// The Skill structure contains information about a character skill.
#[derive(Debug, Clone)]
pub struct Skill {
    PointsRaised: String,
    AdjustPP: String,
    TrainingLevel: String,
    ExperienceSpent: String,
    InnatePoints: String,
    ResistanceOfLastCheck: String,
    LastUsedTime: String
}

// Contains body part table
#[derive(Debug, Clone)]
pub struct Body {
    BodyParts: String
}

// Information on individual body parts. (Needs to be confirmed if this was used in prod)
#[derive(Debug, Clone)]
pub struct BodyPart {
    HasBPSD: String,
    DamageType: String,
    DamageVal: String,
    DamageVar: String,
    ArmorCache: String,
    BH: String,
    BPSD: String
}

// Information on armor levels
#[derive(Debug, Clone)]
pub struct ArmorCache {
    BaseArmor: String,
    ArmorVsSlash: String,
    ArmorVsPierce: String,
    ArmorVsBludgeon: String,
    ArmorVsCold: String,
    ArmorVsFire: String,
    ArmorVsAcid: String,
    ArmorVsElectric: String,
    ArmorVsNether: String
}

#[derive(Debug, Clone)]
pub struct BodyPartSelectionData {
    HLF: String,
    MLF: String,
    LLF: String,
    HRF: String,
    MRF: String,
    LRF: String,
    HLB: String,
    MLB: String,
    LLB: String,
    HRB: String,
    MRB: String,
    LRB: String
}

// Contains information related to the spell in your spellbook
#[derive(Debug, Clone)]
pub struct SpellBookPage {
    CastingLikelihood: String,
    Unknown: String,
    CastingLikelihood2: String
}

// Contains information related to the spells in effect on the character
#[derive(Debug, Clone)]
pub struct EnchantmentRegistry {
    Flags: String,
    LifeSpells: String,
    CreatureSpells: String,
    Vitae: String,
    Cooldowns: String
}

// The Enchantment structure describes an active enchantment.
#[derive(Debug, Clone)]
pub struct Enchantment {
    Id: String,
    HasEquipmentSet: String,
    SpellCategory: String,
    PowerLevel: String,
    StartTime: String,
    Duration: String,
    CasterId: String,
    DegradeModifier: String,
    DegradeLimit: String,
    LastTimeDegraded: String,
    StatMod: String,
    EquipmentSet: String
}

// Information on stat modification
#[derive(Debug, Clone)]
pub struct StatMod {
    Type: String,
    Key: String,
    Value: String
}

// Contains a list of events to filter? Unknown what this does currently.
#[derive(Debug, Clone)]
pub struct EventFilter {
    Events: String
}

// Contains a list of emotes for NPCs? Unknown what this does currently.
#[derive(Debug, Clone)]
pub struct EmoteTable {
    Emotes: String
}

#[derive(Debug, Clone)]
pub struct EmoteSetList {
    Emotes: String
}

#[derive(Debug, Clone)]
pub struct EmoteSet {
    Category: String,
    Probability: String,
    ClassId: String,
    Style: String,
    Substyle: String,
    Quest: String,
    VendorType: String,
    MinHealth: String,
    MaxHealth: String,
    Emotes: String
}

#[derive(Debug, Clone)]
pub struct Emote {
    Type: String,
    Delay: String,
    Extent: String,
    Message: String,
    Message: String,
    Amount: String,
    Stat: String,
    Amount: String,
    Stat: String,
    Stat: String,
    Percent: String,
    Message: String,
    Min: String,
    Max: String,
    Amount64: String,
    HeroXP64: String,
    Amount64: String,
    Amount: String,
    SpellId: String,
    CProfile: String,
    msg: String,
    CProfile: String,
    WealthRating: String,
    TreasureClass: String,
    TreasureType: String,
    Motion: String,
    Frame: String,
    PhysicsScript: String,
    Sound: String,
    Amount: String,
    Stat: String,
    Stat: String,
    Amount: String,
    Message: String,
    Stat: String,
    Message: String,
    TestString: String,
    Stat: String,
    Message: String,
    Min: String,
    Max: String,
    Stat: String,
    Message: String,
    Min64: String,
    Max64: String,
    Stat: String,
    Message: String,
    FMin: String,
    FMax: String,
    Stat: String,
    Percent: String,
    Min64: String,
    Max64: String,
    Stat: String,
    Percent: String,
    Min: String,
    Max: String,
    Display: String,
    Position: String
}

// Set information about an item for creation
#[derive(Debug, Clone)]
pub struct CreationProfile {
    WeenieClassId: String,
    Palette: String,
    Shade: String,
    Destination: String,
    StackSize: String,
    TryToBond: String
}

// List of pages in a book
#[derive(Debug, Clone)]
pub struct PageDataList {
    MaxNumPages: String,
    MaxNumCharsPerPage: String,
    Pages: String
}

// Data for an individual page
#[derive(Debug, Clone)]
pub struct PageData {
    AuthorId: String,
    AuthorName: String,
    AuthorAccount: String,
    Version: String,
    TextIncluded: String,
    IgnoreAuthor: String,
    PageText: String
}

// Blob fragment data used to contruct message data. These can be spread across multiple packets
#[derive(Debug, Clone)]
pub struct BlobFragments {
    Sequence: String,
    Id: String,
    Count: String,
    Size: String,
    Index: String,
    Group: String
}

#[derive(Debug, Clone)]
pub struct GeneratorTable {
    Generators: String
}

#[derive(Debug, Clone)]
pub struct GeneratorProfile {
    Probability: String,
    TypeId: String,
    Delay: String,
    InitCreate: String,
    MaxNum: String,
    WhenCreate: String,
    WhereCreate: String,
    StackSize: String,
    Ptid: String,
    Shade: String,
    PosVal: String,
    Slot: String
}

#[derive(Debug, Clone)]
pub struct GeneratorRegistry {
    Registry: String
}

#[derive(Debug, Clone)]
pub struct GeneratorRegistryNode {
    WcidOrType: String,
    Ts: String,
    TreasureType: String,
    Slot: String,
    Checkpointed: String,
    Shop: String,
    Amount: String
}

// Set of inventory items
#[derive(Debug, Clone)]
pub struct GeneratorQueue {
    Queue: String
}

#[derive(Debug, Clone)]
pub struct GeneratorQueueNode {
    Slot: String,
    When: String
}

#[derive(Debug, Clone)]
pub struct WindowProperty {
    Key_a: String,
    Unknown_c: String,
    TitleSource: String,
    StringId: String,
    FileId: String,
    Value_a: String,
    Unknown_1b: String,
    Unknown_1c: String,
    Unknown_d: String,
    Value_d: String,
    Unknown_e: String,
    Value_e: String,
    Unknown_f: String,
    Value_f: String,
    Unknown_h: String,
    Value_h: String,
    Unknown_i: String,
    Value_i: String,
    Unknown_j: String,
    Value_j: String
}

#[derive(Debug, Clone)]
pub struct WindowOption {
    Type_a: String,
    Unknown_b: String,
    PropertyCount: String
}

#[derive(Debug, Clone)]
pub struct OptionProperty {
    Type: String,
    Unknown_a: String,
    WindowOptions: String,
    unknown_k: String,
    activeOpacity: String,
    unknown_l: String,
    inactiveOpacity: String
}

#[derive(Debug, Clone)]
pub struct GameplayOptions {
    Size: String,
    Unknown200_2: String,
    OptionPropertyCount: String
}

// The PlayerModule structure contains character options.
#[derive(Debug, Clone)]
pub struct PlayerModule {
    Flags: String,
    Options: String,
    Shortcuts: String,
    Tab1Spells: String,
    Tab2Spells: String,
    Tab3Spells: String,
    Tab4Spells: String,
    Tab5Spells: String,
    Tab6Spells: String,
    Tab7Spells: String,
    Tab8Spells: String,
    FillComps: String,
    SpellBookFilters: String,
    OptionFlags: String,
    Unknown100_1: String,
    OptionStrings: String,
    GameplayOptions: String
}

// Set of shortcuts
#[derive(Debug, Clone)]
pub struct ShortCutManager {
    Shortcuts: String
}

// Shortcut
#[derive(Debug, Clone)]
pub struct ShortCutData {
    Index: String,
    ObjectId: String,
    SpellId: String
}

// List of spells in spell tab
#[derive(Debug, Clone)]
pub struct SpellTab {
    Spells: String
}

// Set of inventory items
#[derive(Debug, Clone)]
pub struct ContentProfile {
    ObjectId: String,
    ContainerType: String
}

// Set of inventory items
#[derive(Debug, Clone)]
pub struct InventoryPlacement {
    ObjectId: String,
    Location: String,
    Priority: String
}

// Allegience information
#[derive(Debug, Clone)]
pub struct AllegianceProfile {
    TotalMembers: String,
    TotalVassals: String,
    Hierarchy: String
}

// Allegience record
#[derive(Debug, Clone)]
pub struct AllegianceRecord {
    TreeParent: String,
    AllegianceData: String
}

// Allegience hierarchy information
#[derive(Debug, Clone)]
pub struct AllegianceHierarchy {
    RecordCount: String,
    OldVersion: String,
    Officers: String,
    OfficerTitles: String,
    MonarchBroadcastTime: String,
    MonarchBroadcastsToday: String,
    SpokesBroadcastTime: String,
    SpokesBroadcastsToday: String,
    Motd: String,
    MotdSetBy: String,
    ChatRoomId: String,
    Bindpoint: String,
    AllegianceName: String,
    NameLastSetTime: String,
    IsLocked: String,
    ApprovedVassal: String,
    MonarchData: String
}

// Set of allegiance data for a specific player
#[derive(Debug, Clone)]
pub struct AllegianceData {
    CharacterId: String,
    XPCached: String,
    XPTithed: String,
    Flags: String,
    Gender: String,
    Heritage: String,
    Rank: String,
    Level: String,
    Loyalty: String,
    Leadership: String,
    TimeOnline: String,
    TimeOnline: String,
    AllegianceAge: String,
    Name: String
}

#[derive(Debug, Clone)]
pub struct FriendData {
    FriendId: String,
    Online: String,
    AppearOffline: String,
    Name: String,
    OutFriends: String,
    InFriends: String
}

// Data related to an item, namely the amount and description
#[derive(Debug, Clone)]
pub struct ItemProfile {
    PackedAmount: String,
    ObjectId: String,
    WeenieDescription: String,
    OldWeenieDescription: String
}

// The PublicWeenieDesc structure defines an object's game behavior.
#[derive(Debug, Clone)]
pub struct PublicWeenieDesc {
    Header: String,
    Name: String,
    WeenieClassId: String,
    Icon: String,
    Type: String,
    Behavior: String,
    Header2: String,
    PluralName: String,
    ItemsCapacity: String,
    ContainerCapacity: String,
    AmmunitionType: String,
    Value: String,
    Useability: String,
    UseRadius: String,
    TargetType: String,
    Effects: String,
    CombatUse: String,
    Structure: String,
    MaxStructure: String,
    StackSize: String,
    MaxStackSize: String,
    ContainerId: String,
    WielderId: String,
    ValidSlots: String,
    Slot: String,
    Priority: String,
    BlipColor: String,
    RadarEnum: String,
    PhysicsScript: String,
    Workmanship: String,
    Burden: String,
    SpellId: String,
    OwnerId: String,
    Restrictions: String,
    HookItemTypes: String,
    MonarchId: String,
    HookType: String,
    IconOverlay: String,
    IconUnderlay: String,
    Material: String,
    CooldownId: String,
    CooldownDuration: String,
    PetOwnerId: String
}

// The RestrictionDB contains the access control list for a dwelling object.
#[derive(Debug, Clone)]
pub struct RestrictionDB {
    Version: String,
    Flags: String,
    MonarchId: String,
    Permissions: String
}

// The OldPublicWeenieDesc structure defines an object's game behavior.
#[derive(Debug, Clone)]
pub struct OldPublicWeenieDesc {
    Header: String,
    Name: String,
    WeenieClassId: String,
    Icon: String,
    Type: String,
    Bitfield: String,
    PluralName: String,
    ItemsCapacity: String,
    ContainerCapacity: String,
    Value: String,
    Useability: String,
    UseRadius: String,
    tTargetType: String,
    Effects: String,
    AmmunitionType: String,
    CombatUse: String,
    Structure: String,
    MaxStructure: String,
    StackSize: String,
    MaxStackSize: String,
    ContainerId: String,
    WielderId: String,
    ValidSlots: String,
    Slots: String,
    Priority: String,
    BlipColor: String,
    RadarEnum: String,
    ObviousDistance: String,
    Vndwcid: String,
    SpellId: String,
    HouseOwnerId: String,
    PhysicsScript: String,
    Restrictions: String,
    HookType: String,
    HookItemTypes: String,
    MonarchId: String,
    IconOverlay: String,
    Material: String
}

// Information related to a secure trade.
#[derive(Debug, Clone)]
pub struct Trade {
    PartnerId: String,
    Sequence: String,
    Status: String,
    InitiatorId: String,
    Accepted: String,
    PartnerAccepted: String
}

// A jump with sequences
#[derive(Debug, Clone)]
pub struct JumpPack {
    Extent: String,
    Velocity: String,
    ObjectInstanceSequence: String,
    ObjectServerControlSequence: String,
    ObjectTeleportSequence: String,
    ObjectForcePositionSequence: String
}

// A set of data related to changing states with sequences
#[derive(Debug, Clone)]
pub struct MoveToStatePack {
    RawMotionState: String,
    Position: String,
    ObjectInstanceSequence: String,
    ObjectServerControlSequence: String,
    ObjectTeleportSequence: String,
    ObjectForcePositionSequence: String,
    Contact: String
}

#[derive(Debug, Clone)]
pub struct PackedMotionCommand {
    CommandId: String,
    PackedSequence: String,
    Speed: String
}

// Data related to the movement of the object sent from a client
#[derive(Debug, Clone)]
pub struct RawMotionState {
    Flags: String,
    CurrentHoldkey: String,
    CurrentStyle: String,
    ForwardCommand: String,
    ForwardHoldkey: String,
    ForwardSpeed: String,
    SidestepCommand: String,
    SidestepHoldkey: String,
    SidestepSpeed: String,
    TurnCommand: String,
    TurnHoldkey: String,
    TurnSpeed: String
}

// An autonomous position with sequences
#[derive(Debug, Clone)]
pub struct AutonomousPositionPack {
    Position: String,
    ObjectInstanceSequence: String,
    ObjectServerControlSequence: String,
    ObjectTeleportSequence: String,
    ObjectForcePositionSequence: String,
    Contact: String
}

// A position with sequences
#[derive(Debug, Clone)]
pub struct PositionPack {
    Flags: String,
    Origin: String,
    WQuat: String,
    XQuat: String,
    YQuat: String,
    ZQuat: String,
    Velocity: String,
    PlacementId: String,
    ObjectInstanceSequence: String,
    ObjectPositionSequence: String,
    ObjectTeleportSequence: String,
    ObjectForcePositionSequence: String
}

// Data related to the movement and animation of the object
#[derive(Debug, Clone)]
pub struct MovementData {
    ObjectMovementSequence: String,
    ObjectServerControlSequence: String,
    Autonomous: String,
    MovementType: String,
    OptionFlags: String,
    Stance: String,
    State: String,
    StickyObject: String,
    Target: String,
    Origin: String,
    MoveToParams: String,
    MyRunRate: String,
    Origin: String,
    MoveToParams: String,
    MyRunRate: String,
    TargetId: String,
    DesiredHeading: String,
    TurnToParams: String,
    TurnToParams: String
}

// Contains information for animations and general free motion
#[derive(Debug, Clone)]
pub struct InterpertedMotionState {
    Flags: String,
    CurrentStyle: String,
    ForwardCommand: String,
    SidestepCommand: String,
    TurnCommand: String,
    ForwardSpeed: String,
    SidestepSpeed: String,
    TurnSpeed: String
}

#[derive(Debug, Clone)]
pub struct DDDRevision {
    IdDatFile: String,
    Iteration: String,
    IdsToDownload: String,
    IdsToPurge: String
}

// Set of movement parameters required for a MoveTo movement
#[derive(Debug, Clone)]
pub struct MoveToMovementParameters {
    Bitmember: String,
    DistanceToObject: String,
    MinDistance: String,
    FailDistance: String,
    AnimationSpeed: String,
    WalkRunThreshold: String,
    DesiredHeading: String
}

// Set of movement parameters required for a TurnTo motion
#[derive(Debug, Clone)]
pub struct TurnToMovementParameters {
    Bitmember: String,
    AnimationSpeed: String,
    DesiredHeading: String
}

// The ObjDesc structure defines an object's visual appearance.
#[derive(Debug, Clone)]
pub struct ObjDesc {
    Version: String,
    PaletteCount: String,
    TextureCount: String,
    ModelCount: String,
    Palette: String
}

// Contains data for a subpalette
#[derive(Debug, Clone)]
pub struct Subpalette {
    Palette: String,
    Offset: String,
    NumColors: String
}

// Contains data for texture map changes
#[derive(Debug, Clone)]
pub struct TextureMapChange {
    PartIndex: String,
    OldTexId: String,
    NewTexId: String
}

// Contains data for animation part changes
#[derive(Debug, Clone)]
pub struct AnimPartChange {
    PartIndex: String,
    PartId: String
}

// Data for a character creation
#[derive(Debug, Clone)]
pub struct CharGenResult {
    Account: String,
    One: String,
    HeritageGroup: String,
    Gender: String,
    EyesStrip: String,
    NoseStrip: String,
    MouthStrip: String,
    HairColor: String,
    EyeColor: String,
    HairStyle: String,
    HeadgearStyle: String,
    HeadgearColor: String,
    ShirtStyle: String,
    ShirtColor: String,
    TrousersStyle: String,
    TrousersColor: String,
    FootwearStyle: String,
    FootwearColor: String,
    SkinShade: String,
    HairShade: String,
    HeadgearShade: String,
    ShirtShade: String,
    TrousersShade: String,
    TootwearShade: String,
    TemplateNum: String,
    Strength: String,
    Endurance: String,
    Coordination: String,
    Quickness: String,
    Focus: String,
    Self: String,
    Slot: String,
    ClassId: String,
    Skills: String,
    Name: String,
    StartArea: String,
    IsAdmin: String,
    IsEnvoy: String,
    Validation: String
}

// Basic information for a character used at the Login screen
#[derive(Debug, Clone)]
pub struct CharacterIdentity {
    CharacterId: String,
    Name: String,
    SecondsGreyedOut: String
}

#[derive(Debug, Clone)]
pub struct EquipLocation {
    ObjectId: String,
    Slot: String
}

// The PhysicsDesc structure defines an object's physical behavior.
#[derive(Debug, Clone)]
pub struct PhysicsDesc {
    Flags: String,
    State: String,
    MovementBuffer: String,
    Autonomous: String,
    AnimationFrame: String,
    Position: String,
    MotionId: String,
    SoundId: String,
    PhysicsScriptId: String,
    SetupId: String,
    ParentId: String,
    ParentLocation: String,
    Children: String,
    Scale: String,
    Friction: String,
    Elasticity: String,
    Translucency: String,
    Velocity: String,
    Acceleration: String,
    Omega: String,
    DefaultScript: String,
    DefaultScriptIntensity: String,
    ObjectPositionSequence: String,
    ObjectMovementSequence: String,
    ObjectStateSequence: String,
    ObjectVectorSequence: String,
    ObjectTeleportSequence: String,
    ObjectServerControlSequence: String,
    ObjectForcePositionSequence: String,
    ObjectVisualDescSequence: String,
    ObjectInstanceSequence: String
}

#[derive(Debug, Clone)]
pub struct AdminAccountData {
    AccountName: String,
    BookieId: String
}

#[derive(Debug, Clone)]
pub struct AdminPlayerData {
    name: String,
    bookieId: String
}

#[derive(Debug, Clone)]
pub struct VendorProfile {
    Categories: String,
    MinValue: String,
    MaxValue: String,
    DealsMagic: String,
    BuyPrice: String,
    SellPrice: String,
    CurrencyId: String,
    CurrencyAmount: String,
    CurrencyName: String
}

#[derive(Debug, Clone)]
pub struct ArmorProfile {
    ProtSlashing: String,
    ProtPiercing: String,
    ProtBludgeoning: String,
    ProtCold: String,
    ProtFire: String,
    ProtAcid: String,
    ProtNether: String,
    ProtLightning: String
}

#[derive(Debug, Clone)]
pub struct CreatureAppraisalProfile {
    Flags: String,
    Health: String,
    HealthMax: String,
    Strength: String,
    Endurance: String,
    Quickness: String,
    Coordination: String,
    Focus: String,
    Self: String,
    Stamina: String,
    Mana: String,
    StaminaMax: String,
    ManaMax: String,
    AttrHighlight: String,
    AttrColor: String
}

#[derive(Debug, Clone)]
pub struct WeaponProfile {
    DamageType: String,
    Speed: String,
    Skill: String,
    Damage: String,
    Variance: String,
    Modifier: String,
    Length: String,
    MaxVelocity: String,
    Offsense: String,
    MaxVelocityEstimated: String
}

#[derive(Debug, Clone)]
pub struct HookAppraisalProfile {
    Flags: String,
    ValidLocations: String,
    AmmoType: String
}

#[derive(Debug, Clone)]
pub struct SquelchDB {
    AccountHash: String,
    CharacterHash: String,
    GlobalInfo: String
}

// Set of information related to a squelch entry
#[derive(Debug, Clone)]
pub struct SquelchInfo {
    Filters: String,
    Name: String,
    Account: String
}

// Set of information related to purchasing a housing
#[derive(Debug, Clone)]
pub struct HouseProfile {
    DwellingId: String,
    OwnerId: String,
    Flags: String,
    MinLevel: String,
    MaxLevel: String,
    MinAllegRank: String,
    MaxAllegRank: String,
    MaintenanceFree: String,
    Type: String,
    OwnerName: String,
    Buy: String,
    Rent: String
}

// The HousePayment structure contains information about a house purchase or maintenance item.
#[derive(Debug, Clone)]
pub struct HousePayment {
    Required: String,
    Paid: String,
    WeenieClassId: String,
    Name: String,
    PluralName: String
}

// Set of information related to owning a housing
#[derive(Debug, Clone)]
pub struct HouseData {
    BuyTime: String,
    RentTime: String,
    Type: String,
    MaintenanceFree: String,
    Buy: String,
    Rent: String,
    Position: String
}

// Set of information related to house access
#[derive(Debug, Clone)]
pub struct HAR {
    Version: String,
    Bitmask: String,
    MonarchId: String,
    GuestList: String,
    RoommateList: String
}

// Set of information related to a house guest
#[derive(Debug, Clone)]
pub struct GuestInfo {
    HasStoragePermission: String,
    GuestName: String
}

// Set of information related to a chess game move
#[derive(Debug, Clone)]
pub struct GameMoveData {
    Type: String,
    PlayerId: String,
    Team: String,
    IdPieceToMove: String,
    YGrid: String,
    IdPieceToMove: String,
    YGrid: String,
    XTo: String,
    YTo: String,
    IdPieceToMove: String
}

// Set of information related to a salvage operation
#[derive(Debug, Clone)]
pub struct SalvageOperationsResultData {
    SkillUsed: String,
    NotSalvagable: String,
    SalvageResults: String,
    AugBonus: String
}

// Set of information related to a salvage of an item
#[derive(Debug, Clone)]
pub struct SalvageResult {
    Material: String,
    Workmanship: String,
    Units: String
}

#[derive(Debug, Clone)]
pub struct FellowshipLockData {
    Unknown1: String,
    Unknown2: String,
    Unknown3: String,
    Timestamp: String,
    Sequence: String
}

// Set of information for a fellowship
#[derive(Debug, Clone)]
pub struct Fellowship {
    Members: String,
    Name: String,
    LeaderId: String,
    ShareXP: String,
    EvenXPSplit: String,
    Open: String,
    Locked: String,
    RecentlyDeparted: String,
    Locks: String
}

// The FellowInfo structure contains information about a fellowship member.
#[derive(Debug, Clone)]
pub struct Fellow {
    XPCached: String,
    LumCached: String,
    Level: String,
    MaxHealth: String,
    MaxStamina: String,
    MaxMana: String,
    CurrentHealth: String,
    CurrentStamina: String,
    CurrentMana: String,
    ShareLoot: String,
    Name: String
}

// Contains information about a contract.
#[derive(Debug, Clone)]
pub struct ContractTracker {
    Version: String,
    ContractId: String,
    ContractStage: String,
    TimeWhenDone: String,
    TimeWhenRepeats: String
}

// Contains table of ContractTrackers
#[derive(Debug, Clone)]
pub struct ContractTrackerTable {
    ContactTrackers: String
}

// Set a single character option.
#[derive(Debug, Clone)]
pub struct Character_PlayerOptionChangedEvent {
    Option: String,
    Value: String
}

// Starts a melee attack against a target
#[derive(Debug, Clone)]
pub struct Combat_TargetedMeleeAttack {
    ObjectId: String,
    Height: String,
    Power: String
}

// Starts a missle attack against a target
#[derive(Debug, Clone)]
pub struct Combat_TargetedMissileAttack {
    ObjectId: String,
    Height: String,
    Accuracy: String
}

// Set AFK mode.
#[derive(Debug, Clone)]
pub struct Communication_SetAFKMode {
    AFK: String
}

// Set AFK message.
#[derive(Debug, Clone)]
pub struct Communication_SetAFKMessage {
    Message: String
}

// Talking
#[derive(Debug, Clone)]
pub struct Communication_Talk {
    Message: String
}

// Removes a friend
#[derive(Debug, Clone)]
pub struct Social_RemoveFriend {
    ObjectId: String
}

// Adds a friend
#[derive(Debug, Clone)]
pub struct Social_AddFriend {
    CharacterName: String
}

// Store an item in a container.
#[derive(Debug, Clone)]
pub struct Inventory_PutItemInContainer {
    ObjectId: String,
    ContainerId: String,
    SlotIndex: String
}

// Gets and weilds an item.
#[derive(Debug, Clone)]
pub struct Inventory_GetAndWieldItem {
    ObjectId: String,
    Slot: String
}

// Drop an item.
#[derive(Debug, Clone)]
pub struct Inventory_DropItem {
    ObjectId: String
}

// Swear allegiance
#[derive(Debug, Clone)]
pub struct Allegiance_SwearAllegiance {
    ObjectId: String
}

// Break allegiance
#[derive(Debug, Clone)]
pub struct Allegiance_BreakAllegiance {
    ObjectId: String
}

// Allegiance update request
#[derive(Debug, Clone)]
pub struct Allegiance_UpdateRequest {
    On: String
}

// Sets a character's display title
#[derive(Debug, Clone)]
pub struct Social_SetDisplayCharacterTitle {
    TitleId: String
}

// Direct message by Id
#[derive(Debug, Clone)]
pub struct Communication_TalkDirect {
    Message: String,
    TargetId: String
}

// Sets the allegiance name
#[derive(Debug, Clone)]
pub struct Allegiance_SetAllegianceName {
    Name: String
}

// Attempt to use an item with a target.
#[derive(Debug, Clone)]
pub struct Inventory_UseWithTargetEvent {
    ObjectId: String,
    TargetId: String
}

// Attempt to use an item.
#[derive(Debug, Clone)]
pub struct Inventory_UseEvent {
    ObjectId: String
}

// Sets an allegiance officer
#[derive(Debug, Clone)]
pub struct Allegiance_SetAllegianceOfficer {
    CharacterName: String,
    Level: String
}

// Sets an allegiance officer title for a given level
#[derive(Debug, Clone)]
pub struct Allegiance_SetAllegianceOfficerTitle {
    Level: String,
    Title: String
}

// Perform the allegiance lock action
#[derive(Debug, Clone)]
pub struct Allegiance_DoAllegianceLockAction {
    Action: String
}

// Sets a person as an approved vassal
#[derive(Debug, Clone)]
pub struct Allegiance_SetAllegianceApprovedVassal {
    CharacterName: String
}

// Gags a person in allegiance chat
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceChatGag {
    CharacterName: String,
    On: String
}

// Perform the allegiance house action
#[derive(Debug, Clone)]
pub struct Allegiance_DoAllegianceHouseAction {
    Action: String
}

// Spend XP to raise a vital.
#[derive(Debug, Clone)]
pub struct Train_TrainAttribute2nd {
    Type: String,
    Experience: String
}

// Spend XP to raise an attribute.
#[derive(Debug, Clone)]
pub struct Train_TrainAttribute {
    Type: String,
    Experience: String
}

// Spend XP to raise a skill.
#[derive(Debug, Clone)]
pub struct Train_TrainSkill {
    Skill: String,
    Experience: String
}

// Spend skill credits to train a skill.
#[derive(Debug, Clone)]
pub struct Train_TrainSkillAdvancementClass {
    Skill: String,
    Credits: String
}

// Cast a spell with no target.
#[derive(Debug, Clone)]
pub struct Magic_CastUntargetedSpell {
    SpellId: String
}

// Cast a spell on a target
#[derive(Debug, Clone)]
pub struct Magic_CastTargetedSpell {
    ObjectId: String,
    SpellId: String
}

// Changes the combat mode
#[derive(Debug, Clone)]
pub struct Combat_ChangeCombatMode {
    Mode: String
}

// Merges one stack with another
#[derive(Debug, Clone)]
pub struct Inventory_StackableMerge {
    ObjectId: String,
    TargetId: String,
    Amount: String
}

// Split a stack and place it into a container
#[derive(Debug, Clone)]
pub struct Inventory_StackableSplitToContainer {
    ObjectId: String,
    ContainerId: String,
    SlotIndex: String,
    Amount: String
}

// Split a stack and place it into the world
#[derive(Debug, Clone)]
pub struct Inventory_StackableSplitTo3D {
    ObjectId: String,
    Amount: String
}

// Changes an account squelch
#[derive(Debug, Clone)]
pub struct Communication_ModifyCharacterSquelch {
    Add: String,
    ObjectId: String,
    CharacterName: String,
    Type: String
}

// Changes an account squelch
#[derive(Debug, Clone)]
pub struct Communication_ModifyAccountSquelch {
    Add: String,
    CharacterName: String
}

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Debug, Clone)]
pub struct Communication_ModifyGlobalSquelch {
    Add: String,
    Type: String
}

// Direct message by name
#[derive(Debug, Clone)]
pub struct Communication_TalkDirectByName {
    Message: String,
    TargetName: String
}

// Buy from a vendor
#[derive(Debug, Clone)]
pub struct Vendor_Buy {
    ObjectId: String,
    Items: String,
    AlternateCurrencyId: String
}

// Sell to a vendor
#[derive(Debug, Clone)]
pub struct Vendor_Sell {
    ObjectId: String,
    Items: String
}

// Create a fellowship
#[derive(Debug, Clone)]
pub struct Fellowship_Create {
    Name: String,
    ShareXP: String
}

// Quit the fellowship
#[derive(Debug, Clone)]
pub struct Fellowship_Quit {
    Disband: String
}

// Dismiss a player from the fellowship
#[derive(Debug, Clone)]
pub struct Fellowship_Dismiss {
    ObjectId: String
}

// Recruit a player to the fellowship
#[derive(Debug, Clone)]
pub struct Fellowship_Recruit {
    ObjectId: String
}

// Update request
#[derive(Debug, Clone)]
pub struct Fellowship_UpdateRequest {
    On: String
}

// Request update to book data (seems to be sent after failed add page)
#[derive(Debug, Clone)]
pub struct Writing_BookAddPage {
    ObjectId: String
}

// Updates a page in a book
#[derive(Debug, Clone)]
pub struct Writing_BookModifyPage {
    ObjectId: String,
    PageNum: String,
    PageText: String
}

// Add a page to a book
#[derive(Debug, Clone)]
pub struct Writing_BookData {
    ObjectId: String
}

// Removes a page from a book
#[derive(Debug, Clone)]
pub struct Writing_BookDeletePage {
    ObjectId: String,
    PageNum: String
}

// Requests data for a page in a book
#[derive(Debug, Clone)]
pub struct Writing_BookPageData {
    ObjectId: String,
    PageNum: String
}

// Sets the inscription on an object
#[derive(Debug, Clone)]
pub struct Writing_SetInscription {
    ObjectId: String,
    Inscription: String
}

// Appraise something
#[derive(Debug, Clone)]
pub struct Item_Appraise {
    ObjectId: String
}

// Give an item to someone.
#[derive(Debug, Clone)]
pub struct Inventory_GiveObjectRequest {
    TargetId: String,
    ObjectId: String,
    Amount: String
}

// Advocate Teleport
#[derive(Debug, Clone)]
pub struct Advocate_Teleport {
    ObjectId: String,
    Destination: String
}

// Sends an abuse report.
#[derive(Debug, Clone)]
pub struct Character_AbuseLogRequest {
    Character: String,
    Status: String,
    Complaint: String
}

// Joins a chat channel
#[derive(Debug, Clone)]
pub struct Communication_AddToChannel {
    Channel: String
}

// Leaves a chat channel
#[derive(Debug, Clone)]
pub struct Communication_RemoveFromChannel {
    Channel: String
}

// Sends a message to a chat channel
#[derive(Debug, Clone)]
pub struct Communication_ChannelBroadcast {
    Channel: String,
    SenderName: String,
    Message: String
}

#[derive(Debug, Clone)]
pub struct Communication_ChannelList {
    Channel: String
}

// Stop viewing the contents of a container
#[derive(Debug, Clone)]
pub struct Inventory_NoLongerViewingContents {
    ObjectId: String
}

// Splits an item to a wield location.
#[derive(Debug, Clone)]
pub struct Inventory_StackableSplitToWield {
    ObjectId: String,
    Slot: String,
    Amount: String
}

// Add an item to the shortcut bar.
#[derive(Debug, Clone)]
pub struct Character_AddShortCut {
    Shortcut: String
}

// Remove an item from the shortcut bar.
#[derive(Debug, Clone)]
pub struct Character_RemoveShortCut {
    Index: String
}

// Set multiple character options.
#[derive(Debug, Clone)]
pub struct Character_CharacterOptionsEvent {
    Options: String
}

// Removes a spell from the spell book
#[derive(Debug, Clone)]
pub struct Magic_RemoveSpell {
    SpellId: String
}

// Query's a creatures health
#[derive(Debug, Clone)]
pub struct Combat_QueryHealth {
    ObjectId: String
}

// Query a character's age
#[derive(Debug, Clone)]
pub struct Character_QueryAge {
    ObjectId: String
}

// Query a character's birth day
#[derive(Debug, Clone)]
pub struct Character_QueryBirth {
    ObjectId: String
}

// Emote message
#[derive(Debug, Clone)]
pub struct Communication_Emote {
    Message: String
}

// Soul emote message
#[derive(Debug, Clone)]
pub struct Communication_SoulEmote {
    Message: String
}

// Add a spell to a spell bar.
#[derive(Debug, Clone)]
pub struct Character_AddSpellFavorite {
    SpellId: String,
    Index: String,
    SpellBar: String
}

// Remove a spell from a spell bar.
#[derive(Debug, Clone)]
pub struct Character_RemoveSpellFavorite {
    SpellId: String,
    SpellBar: String
}

// Starts trading with another player.
#[derive(Debug, Clone)]
pub struct Trade_OpenTradeNegotiations {
    ObjectId: String
}

// Adds an object to the trade.
#[derive(Debug, Clone)]
pub struct Trade_AddToTrade {
    ObjectId: String,
    SlotIndex: String
}

// Accepts a trade.
#[derive(Debug, Clone)]
pub struct Trade_AcceptTrade {
    Contents: String
}

// Remove your corpse looting permission for the given player, /consent remove 
#[derive(Debug, Clone)]
pub struct Character_RemoveFromPlayerConsentList {
    TargetName: String
}

// Grants a player corpse looting permission, /permit add
#[derive(Debug, Clone)]
pub struct Character_AddPlayerPermission {
    TargetName: String
}

// Buy a house
#[derive(Debug, Clone)]
pub struct House_BuyHouse {
    ObjectId: String,
    Items: String
}

// Revokes a player's corpse looting permission, /permit remove
#[derive(Debug, Clone)]
pub struct Character_RemovePlayerPermission {
    TargetName: String
}

// Pay rent for a house
#[derive(Debug, Clone)]
pub struct House_RentHouse {
    ObjectId: String,
    Items: String
}

// Sets a new fill complevel for a component
#[derive(Debug, Clone)]
pub struct Character_SetDesiredComponentLevel {
    Wcid: String,
    Amount: String
}

// Adds a guest to your house guest list
#[derive(Debug, Clone)]
pub struct House_AddPermanentGuest {
    GuestName: String
}

// Removes a specific player from your house guest list, /house guest remove
#[derive(Debug, Clone)]
pub struct House_RemovePermanentGuest {
    GuestName: String
}

// Sets your house open status
#[derive(Debug, Clone)]
pub struct House_SetOpenHouseStatus {
    OpenHouse: String
}

// Changes a specific players storage permission, /house storage add/remove
#[derive(Debug, Clone)]
pub struct House_ChangeStoragePermission {
    GuestName: String,
    HasPermission: String
}

// Boots a specific player from your house /house boot
#[derive(Debug, Clone)]
pub struct House_BootSpecificHouseGuest {
    GuestName: String
}

// Sets the allegiance message of the day, /allegiance motd set
#[derive(Debug, Clone)]
pub struct Allegiance_SetMotd {
    Message: String
}

// Gets SlumLord info, sent after getting a failed house transaction
#[derive(Debug, Clone)]
pub struct House_QueryLord {
    ObjectId: String
}

// Queries an item's mana
#[derive(Debug, Clone)]
pub struct Item_QueryItemMana {
    ObjectId: String
}

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Debug, Clone)]
pub struct House_SetHooksVisibility {
    Visible: String
}

// Modify whether allegiance members are guests, /house guest add_allegiance/remove_allegiance
#[derive(Debug, Clone)]
pub struct House_ModifyAllegianceGuestPermission {
    Add: String
}

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Debug, Clone)]
pub struct House_ModifyAllegianceStoragePermission {
    Add: String
}

// Joins a chess game
#[derive(Debug, Clone)]
pub struct Game_Join {
    GameId: String,
    Team: String
}

// Makes a chess move
#[derive(Debug, Clone)]
pub struct Game_Move {
    XFrom: String,
    YFrom: String,
    XTo: String,
    YTo: String
}

// Offer or confirm stalemate
#[derive(Debug, Clone)]
pub struct Game_Stalemate {
    On: String
}

// Lists available house /house available
#[derive(Debug, Clone)]
pub struct House_ListAvailableHouses {
    Type: String
}

// Confirms a dialog
#[derive(Debug, Clone)]
pub struct Character_ConfirmationResponse {
    Type: String,
    Context: String,
    Accepted: String
}

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Debug, Clone)]
pub struct Allegiance_BreakAllegianceBoot {
    BooteeName: String,
    AccountBoot: String
}

// Request allegiance info for a player
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceInfoRequest {
    TargetName: String
}

// Salvages items
#[derive(Debug, Clone)]
pub struct Inventory_CreateTinkeringTool {
    ToolId: String,
    Items: String
}

// Changes the spell book filter
#[derive(Debug, Clone)]
pub struct Character_SpellbookFilterEvent {
    Options: String
}

// Fellowship Assign a new leader
#[derive(Debug, Clone)]
pub struct Fellowship_AssignNewLeader {
    ObjectId: String
}

// Fellowship Change openness
#[derive(Debug, Clone)]
pub struct Fellowship_ChangeFellowOpeness {
    Open: String
}

// Boots a player from the allegiance chat
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceChatBoot {
    CharacterName: String,
    Reason: String
}

// Bans a player from the allegiance
#[derive(Debug, Clone)]
pub struct Allegiance_AddAllegianceBan {
    CharacterName: String
}

// Removes a player ban from the allegiance
#[derive(Debug, Clone)]
pub struct Allegiance_RemoveAllegianceBan {
    CharacterName: String
}

// Removes an allegiance officer
#[derive(Debug, Clone)]
pub struct Allegiance_RemoveAllegianceOfficer {
    CharacterName: String
}

// Admin Returns a plugin list response
#[derive(Debug, Clone)]
pub struct Admin_QueryPluginListResponse {
    Context: String,
    PluginList: String
}

// Admin Returns plugin info
#[derive(Debug, Clone)]
pub struct Admin_QueryPluginResponse {
    Context: String,
    Success: String,
    PluginName: String,
    PluginAuthor: String,
    PluginEmail: String,
    PluginWebpage: String
}

// Completes the barber interaction
#[derive(Debug, Clone)]
pub struct Character_FinishBarber {
    BasePalette: String,
    HeadObject: String,
    HeadTexture: String,
    DefaultHeadTexture: String,
    EyesTexture: String,
    DefaultEyesTexture: String,
    NoseTexture: String,
    DefaultNoseTexture: String,
    MouthTexture: String,
    DefaultMouthTexture: String,
    SkinPalette: String,
    HairPalette: String,
    EyesPalette: String,
    SetupId: String,
    Option1: String,
    Option2: String
}

// Abandons a contract
#[derive(Debug, Clone)]
pub struct Social_AbandonContract {
    ContractId: String
}

// Performs a jump
#[derive(Debug, Clone)]
pub struct Movement_Jump {
    Jump: String
}

// Move to state data
#[derive(Debug, Clone)]
pub struct Movement_MoveToState {
    MoveToState: String
}

// Performs a movement based on input
#[derive(Debug, Clone)]
pub struct Movement_DoMovementCommand {
    Motion: String,
    Speed: String,
    HoldKey: String
}

// Stops a movement
#[derive(Debug, Clone)]
pub struct Movement_StopMovementCommand {
    Motion: String,
    HoldKey: String
}

// Sets an autonomy level
#[derive(Debug, Clone)]
pub struct Movement_AutonomyLevel {
    AutonomyLevel: String
}

// Sends an autonomous position
#[derive(Debug, Clone)]
pub struct Movement_AutonomousPosition {
    Position: String
}

// Performs a non autonomous jump
#[derive(Debug, Clone)]
pub struct Movement_Jump_NonAutonomous {
    Extent: String
}

// Allegiance update cancelled
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceUpdateAborted {
    FailureType: String
}

// Display a message in a popup message window.
#[derive(Debug, Clone)]
pub struct Communication_PopUpString {
    Message: String
}

// Information describing your character.
#[derive(Debug, Clone)]
pub struct Login_PlayerDescription {
    BaseQualities: String,
    Qualities: String,
    PlayerModule: String,
    ContentProfile: String,
    InventoryPlacement: String
}

// Returns info related to your monarch, patron and vassals.
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceUpdate {
    Rank: String,
    Profile: String
}

// Friends list update
#[derive(Debug, Clone)]
pub struct Social_FriendsUpdate {
    Friends: String,
    Type: String
}

// Store an item in a container.
#[derive(Debug, Clone)]
pub struct Item_ServerSaysContainId {
    ObjectId: String,
    ContainerId: String,
    SlotIndex: String,
    ContainerType: String
}

// Equip an item.
#[derive(Debug, Clone)]
pub struct Item_WearItem {
    ObjectId: String,
    Slot: String
}

// Titles for the current character.
#[derive(Debug, Clone)]
pub struct Social_CharacterTitleTable {
    DisplayTitle: String,
    Titles: String
}

// Set a title for the current character.
#[derive(Debug, Clone)]
pub struct Social_AddOrSetCharacterTitle {
    NewTitle: String,
    SetAsDisplayTitle: String
}

// Close Container - Only sent when explicitly closed
#[derive(Debug, Clone)]
pub struct Item_StopViewingObjectContents {
    ObjectId: String
}

// Open the buy/sell panel for a merchant.
#[derive(Debug, Clone)]
pub struct Vendor_VendorInfo {
    ObjectId: String,
    Profile: String,
    Items: String
}

// Opens barber UI
#[derive(Debug, Clone)]
pub struct Character_StartBarber {
    BasePalette: String,
    HeadObject: String,
    HeadTexture: String,
    DefaultHeadTexture: String,
    EyesTexture: String,
    DefaultEyesTexture: String,
    NoseTexture: String,
    DefaultNoseTexture: String,
    MouthTexture: String,
    DefaultMouthTexture: String,
    SkinPalette: String,
    HairPalette: String,
    EyesPalette: String,
    SetupId: String,
    Option1: String,
    Option2: String
}

// Member left fellowship
#[derive(Debug, Clone)]
pub struct Fellowship_Quit {
    Disband: String
}

// Member dismissed from fellowship
#[derive(Debug, Clone)]
pub struct Fellowship_Dismiss {
    ObjectId: String
}

// Sent when you first open a book, contains the entire table of contents.
#[derive(Debug, Clone)]
pub struct Writing_BookOpen {
    BookId: String,
    MaxNumPages: String,
    PageData: String,
    Inscription: String,
    ScribeId: String,
    ScribeName: String
}

// Response to an attempt to add a page to a book.
#[derive(Debug, Clone)]
pub struct Writing_BookAddPageResponse {
    BookId: String,
    PageNumber: String,
    Success: String
}

// Response to an attempt to delete a page from a book.
#[derive(Debug, Clone)]
pub struct Writing_BookDeletePageResponse {
    BookId: String,
    PageNumber: String,
    Success: String
}

// Contains the text of a single page of a book, parchment or sign.
#[derive(Debug, Clone)]
pub struct Writing_BookPageDataResponse {
    ObjectId: String,
    Page: String,
    PageData: String
}

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Debug, Clone)]
pub struct Item_GetInscriptionResponse {
    Inscription: String,
    ScribeName: String,
    ScribeAccount: String
}

// The result of an attempt to assess an item or creature.
#[derive(Debug, Clone)]
pub struct Item_SetAppraiseInfo {
    ObjectId: String,
    Flags: String,
    Success: String,
    IntProperties: String,
    Int64Properties: String,
    BoolProperties: String,
    FloatProperties: String,
    StringProperties: String,
    DataIdProperties: String,
    SpellBook: String,
    ArmorProfile: String,
    CreatureProfile: String,
    WeaponProfile: String,
    HookProfile: String,
    ArmorHighlight: String,
    ArmorColor: String,
    WeaponHighlight: String,
    WeaponColor: String,
    ResistHighlight: String,
    ResistColor: String,
    BaseArmorHead: String,
    BaseArmorChest: String,
    BaseArmorGroin: String,
    BaseArmorBicep: String,
    BaseArmorWrist: String,
    BaseArmorHand: String,
    BaseArmorThigh: String,
    BaseArmorShin: String,
    BaseArmorFoot: String
}

// ChannelBroadcast: Group Chat
#[derive(Debug, Clone)]
pub struct Communication_ChannelBroadcast {
    Channel: String,
    Message: String
}

// ChannelList: Provides list of characters listening to a channel, I assume in response to a command
#[derive(Debug, Clone)]
pub struct Communication_ChannelList {
    Characters: String
}

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Debug, Clone)]
pub struct Communication_ChannelIndex {
    Channels: String
}

// Set Pack Contents
#[derive(Debug, Clone)]
pub struct Item_OnViewContents {
    ContainerId: String,
    Items: String
}

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Debug, Clone)]
pub struct Item_ServerSaysMoveItem {
    ObjectId: String
}

// HandleAttackDoneEvent: Melee attack completed
#[derive(Debug, Clone)]
pub struct Combat_HandleAttackDoneEvent {
    Number: String
}

// RemoveSpell: Delete a spell from your spellbook.
#[derive(Debug, Clone)]
pub struct Magic_RemoveSpell {
    SpellId: String
}

// You just died.
#[derive(Debug, Clone)]
pub struct Combat_HandleVictimNotificationEventSelf {
    Message: String
}

// Message for a death, something you killed or your own death message.
#[derive(Debug, Clone)]
pub struct Combat_HandleVictimNotificationEventOther {
    Message: String
}

// HandleAttackerNotificationEvent: You have hit your target with a melee attack.
#[derive(Debug, Clone)]
pub struct Combat_HandleAttackerNotificationEvent {
    DefenderName: String,
    Type: String,
    DamagePercent: String,
    Damage: String,
    Critical: String,
    AttackConditions: String
}

// HandleDefenderNotificationEvent: You have been hit by a creature's melee attack.
#[derive(Debug, Clone)]
pub struct Combat_HandleDefenderNotificationEvent {
    AttackerName: String,
    Type: String,
    DamagePercent: String,
    Damage: String,
    Location: String,
    Critical: String,
    AttackConditions: String
}

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Debug, Clone)]
pub struct Combat_HandleEvasionAttackerNotificationEvent {
    DefenderName: String
}

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Debug, Clone)]
pub struct Combat_HandleEvasionDefenderNotificationEvent {
    AttackerName: String
}

// QueryHealthResponse: Update a creature's health bar.
#[derive(Debug, Clone)]
pub struct Combat_QueryHealthResponse {
    ObjectId: String,
    Health: String
}

// QueryAgeResponse: happens when you do /age in the game
#[derive(Debug, Clone)]
pub struct Character_QueryAgeResponse {
    TargetName: String,
    Age: String
}

// UseDone: Ready. Previous action complete
#[derive(Debug, Clone)]
pub struct Item_UseDone {
    FailureType: String
}

// Allegiance update finished
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceUpdateDone {
    FailureType: String
}

// Close Assess Panel
#[derive(Debug, Clone)]
pub struct Item_AppraiseDone {
    Unknown: String
}

// Squelch and Filter List
#[derive(Debug, Clone)]
pub struct Communication_SetSquelchDB {
    SquelchDB: String
}

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Debug, Clone)]
pub struct Trade_RegisterTrade {
    InitiatorId: String,
    PartnerId: String,
    Stamp: String
}

// OpenTrade: Open trade window
#[derive(Debug, Clone)]
pub struct Trade_OpenTrade {
    ObjectId: String
}

// CloseTrade: End trading
#[derive(Debug, Clone)]
pub struct Trade_CloseTrade {
    Reason: String
}

// RemoveFromTrade: Item was removed from trade window
#[derive(Debug, Clone)]
pub struct Trade_AddToTrade {
    ObjectId: String,
    Side: String
}

// Removes an item from the trade window, not sure if this is used still?
#[derive(Debug, Clone)]
pub struct Trade_RemoveFromTrade {
    ObjectId: String,
    Side: String
}

// AcceptTrade: The trade was accepted
#[derive(Debug, Clone)]
pub struct Trade_AcceptTrade {
    ObjectId: String
}

// DeclineTrade: The trade was declined
#[derive(Debug, Clone)]
pub struct Trade_DeclineTrade {
    ObjectId: String
}

// ResetTrade: The trade window was reset
#[derive(Debug, Clone)]
pub struct Trade_ResetTrade {
    ObjectId: String
}

// TradeFailure: Failure to add a trade item
#[derive(Debug, Clone)]
pub struct Trade_TradeFailure {
    ObjectId: String,
    Reason: String
}

// Buy a dwelling or pay maintenance
#[derive(Debug, Clone)]
pub struct House_HouseProfile {
    ObjectId: String,
    Profile: String
}

// House panel information for owners.
#[derive(Debug, Clone)]
pub struct House_HouseData {
    Data: String
}

// House Data
#[derive(Debug, Clone)]
pub struct House_HouseStatus {
    NoticeType: String
}

// Update Rent Time
#[derive(Debug, Clone)]
pub struct House_UpdateRentTime {
    RentTime: String
}

// Update Rent Payment
#[derive(Debug, Clone)]
pub struct House_UpdateRentPayment {
    Rent: String
}

// Update Restrictions
#[derive(Debug, Clone)]
pub struct House_UpdateRestrictions {
    Sequence: String,
    SenderId: String,
    Restrictions: String
}

// House Guest List
#[derive(Debug, Clone)]
pub struct House_UpdateHAR {
    GuestList: String
}

// House Profile
#[derive(Debug, Clone)]
pub struct House_HouseTransaction {
    NoticeType: String
}

// Update an item's mana bar.
#[derive(Debug, Clone)]
pub struct Item_QueryItemManaResponse {
    ObjectId: String,
    Mana: String,
    Success: String
}

// Display a list of available dwellings in the chat window.
#[derive(Debug, Clone)]
pub struct House_AvailableHouses {
    Type: String,
    Houses: String,
    NumHouses: String
}

// Display a confirmation panel.
#[derive(Debug, Clone)]
pub struct Character_ConfirmationRequest {
    ConfirmationType: String,
    ContextId: String,
    Text: String
}

// Confirmation done
#[derive(Debug, Clone)]
pub struct Character_ConfirmationDone {
    ConfirmationType: String,
    ContextId: String
}

// Display an allegiance login/logout message in the chat window.
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceLoginNotificationEvent {
    CharacterId: String,
    IsLoggedIn: String
}

// Returns data for a player's allegiance information
#[derive(Debug, Clone)]
pub struct Allegiance_AllegianceInfoResponseEvent {
    TargetId: String,
    Profile: String
}

// Joining game response
#[derive(Debug, Clone)]
pub struct Game_JoinGameResponse {
    GameId: String,
    Team: String
}

// Start game
#[derive(Debug, Clone)]
pub struct Game_StartGame {
    GameId: String,
    Team: String
}

// Move response
#[derive(Debug, Clone)]
pub struct Game_MoveResponse {
    GameId: String,
    MoveResult: String
}

// Opponent Turn
#[derive(Debug, Clone)]
pub struct Game_OpponentTurn {
    GameId: String,
    Team: String,
    GameMove: String
}

// Opponent Stalemate State
#[derive(Debug, Clone)]
pub struct Game_OpponentStalemateState {
    GameId: String,
    Team: String,
    On: String
}

// Display a status message in the chat window.
#[derive(Debug, Clone)]
pub struct Communication_WeenieError {
    Type: String
}

// Display a parameterized status message in the chat window.
#[derive(Debug, Clone)]
pub struct Communication_WeenieErrorWithString {
    Type: String,
    Text: String
}

// End of Chess game
#[derive(Debug, Clone)]
pub struct Game_GameOver {
    GameId: String,
    TeamWinner: String
}

// Set Turbine Chat channel numbers.
#[derive(Debug, Clone)]
pub struct Communication_ChatRoomTracker {
    AllegianceRoomId: String,
    GeneralChatRoomId: String,
    TradeChatRoomId: String,
    LFGChatRoomId: String,
    RoleplayChatRoomId: String,
    OlthoiChatRoomId: String,
    SocietyChatRoomId: String,
    SocietyCelestialHandChatRoomId: String,
    SocietyEldrichWebChatRoomId: String,
    SocietyRadiantBloodChatRoomId: String
}

// Salvage operation results
#[derive(Debug, Clone)]
pub struct Inventory_SalvageOperationsResultData {
    Result: String
}

// Someone has sent you a @tell.
#[derive(Debug, Clone)]
pub struct Communication_HearDirectSpeech {
    Message: String,
    SenderName: String,
    SenderId: String,
    TargetId: String,
    Type: String,
    SecretFlags: String
}

// Create or join a fellowship
#[derive(Debug, Clone)]
pub struct Fellowship_FullUpdate {
    Fellowship: String
}

// Add/Update a member to your fellowship.
#[derive(Debug, Clone)]
pub struct Fellowship_UpdateFellow {
    Fellow: String,
    UpdateType: String
}

// Add a spell to your spellbook.
#[derive(Debug, Clone)]
pub struct Magic_UpdateSpell {
    SpellId: String
}

// Apply an enchantment to your character.
#[derive(Debug, Clone)]
pub struct Magic_UpdateEnchantment {
    Enchantment: String
}

// Remove an enchantment from your character.
#[derive(Debug, Clone)]
pub struct Magic_RemoveEnchantment {
    SpellId: String
}

// Update multiple enchantments from your character.
#[derive(Debug, Clone)]
pub struct Magic_UpdateMultipleEnchantments {
    Enchantments: String
}

// Remove multiple enchantments from your character.
#[derive(Debug, Clone)]
pub struct Magic_RemoveMultipleEnchantments {
    Enchantments: String
}

// Silently remove An enchantment from your character.
#[derive(Debug, Clone)]
pub struct Magic_DispelEnchantment {
    SpellId: String
}

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Debug, Clone)]
pub struct Magic_DispelMultipleEnchantments {
    Enchantments: String
}

// A portal storm is brewing.
#[derive(Debug, Clone)]
pub struct Misc_PortalStormBrewing {
    Extent: String
}

// A portal storm is imminent.
#[derive(Debug, Clone)]
pub struct Misc_PortalStormImminent {
    Extent: String
}

// Display a status message on the Action Viewscreen (the red text overlaid on the 3D area).
#[derive(Debug, Clone)]
pub struct Communication_TransientString {
    Message: String
}

// Sends all contract data
#[derive(Debug, Clone)]
pub struct Social_SendClientContractTrackerTable {
    ContractTracker: String
}

// Updates a contract data
#[derive(Debug, Clone)]
pub struct Social_SendClientContractTracker {
    ContractTracker: String,
    DeleteContract: String,
    SetAsDisplayContract: String
}

// Mark a character for deletetion.
#[derive(Debug, Clone)]
pub struct Character_CharacterDelete {
    Account: String,
    Slot: String
}

// Character creation result
#[derive(Debug, Clone)]
pub struct Character_SendCharGenResult {
    Account: String,
    Result: String
}

// The character to log in.
#[derive(Debug, Clone)]
pub struct Login_SendEnterWorld {
    CharacterId: String,
    Account: String
}

// Asks server for a new object description
#[derive(Debug, Clone)]
pub struct Object_SendForceObjdesc {
    ObjectId: String
}

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Debug, Clone)]
pub struct Social_SendFriendsCommand {
    Command: String,
    Player: String
}

// Admin command to restore a character
#[derive(Debug, Clone)]
pub struct Admin_SendAdminRestoreCharacter {
    ObjectId: String,
    RestoredCharName: String,
    AccountToRestoreTo: String
}

// Send or receive a message using Turbine Chat.
#[derive(Debug, Clone)]
pub struct Communication_TurbineChat {
    MmessageSize: String,
    Type: String,
    BlobDispatchType: String,
    TargetType: String,
    TargetId: String,
    TransportType: String,
    TransportId: String,
    Cookie: String,
    PayloadSize: String,
    RoomId: String,
    DisplayName: String,
    Text: String,
    ExtraDataSize: String,
    SpeakerId: String,
    HResult: String,
    ChatType: String,
    ContextId: String,
    ResponseId: String,
    MethodId: String,
    RoomId: String,
    Text: String,
    ExtraDataSize: String,
    SpeakerId: String,
    HResult: String,
    ChatType: String,
    ContextId: String,
    ResponseId: String,
    MethodId: String,
    HResult: String,
    ContextId: String,
    ResponseId: String,
    MethodId: String,
    HResult: String
}

// DDD request for data
#[derive(Debug, Clone)]
pub struct DDD_RequestDataMessage {
    ResourceType: String,
    ResourceId: String
}

// TODO
#[derive(Debug, Clone)]
pub struct DDD_InterrogationResponseMessage {
    Language: String,
    Files: String
}

// Sent every time an object you are aware of ceases to exist. Merely running out of range does not generate this message - in that case, the client just automatically stops tracking it after receiving no updates for a while (which I presume is a very short while).
#[derive(Debug, Clone)]
pub struct Item_ServerSaysRemove {
    ObjectId: String
}

// Failure to give an item
#[derive(Debug, Clone)]
pub struct Character_ServerSaysAttemptFailed {
    ObjectId: String,
    Reason: String
}

// For stackable items, this changes the number of items in the stack.
#[derive(Debug, Clone)]
pub struct Item_UpdateStackSize {
    Sequence: String,
    ObjectId: String,
    Amount: String,
    NewValue: String
}

// A Player Kill occurred nearby (also sent for suicides).
#[derive(Debug, Clone)]
pub struct Combat_HandlePlayerDeathEvent {
    Message: String,
    KilledId: String,
    KillerId: String
}

// Remove an int property from the character
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemoveIntEvent {
    Sequence: String,
    Type: String
}

// Remove an int property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemoveIntEvent {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Remove a bool property from the charactert
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemoveBoolEvent {
    Sequence: String,
    Type: String
}

// Remove a bool property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemoveBoolEvent {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Remove a float property from the character
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemoveFloatEvent {
    Sequence: String,
    Type: String
}

// Remove a float property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemoveFloatEvent {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Remove a string property from the character
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemoveStringEvent {
    Sequence: String,
    Type: String
}

// Remove a string property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemoveStringEvent {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Remove an dataId property from the character
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemoveDataIdEvent {
    Sequence: String,
    Type: String
}

// Remove an dataId property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemoveDataIdEvent {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Remove an instanceId property from the character
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemoveInstanceIdEvent {
    Sequence: String,
    Type: String
}

// Remove an instanceId property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemoveInstanceIdEvent {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Remove a position property from the character
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemovePositionEvent {
    Sequence: String,
    Type: String
}

// Remove a position property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemovePositionEvent {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Remove an int64 property from the character
#[derive(Debug, Clone)]
pub struct Qualities_PrivateRemoveInt64Event {
    Sequence: String,
    Type: String
}

// Remove an int64 property from an object
#[derive(Debug, Clone)]
pub struct Qualities_RemoveInt64Event {
    Sequence: String,
    ObjectId: String,
    Type: String
}

// Set or update a Character Int property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateInt {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update an Object Int property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateInt {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Int64 property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateInt64 {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Int64 property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateInt64 {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Boolean property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateBool {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update an Object Boolean property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateBool {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update an Object float property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateFloat {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update an Object float property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateFloat {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update an Object String property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateString {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update an Object String property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateString {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update an Object DId property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateDataId {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update an Object DId property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateDataId {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a IId property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateInstanceId {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update an Object IId property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateInstanceId {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Position property value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdatePosition {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Position property value
#[derive(Debug, Clone)]
pub struct Qualities_UpdatePosition {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Skill value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateSkill {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Skill value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateSkill {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Skill Level
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateSkillLevel {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Skill Level
#[derive(Debug, Clone)]
pub struct Qualities_UpdateSkillLevel {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Skill state
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateSkillAC {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Skill state
#[derive(Debug, Clone)]
pub struct Qualities_UpdateSkillAC {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Attribute value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateAttribute {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Attribute value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateAttribute {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Attribute Level
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateAttributeLevel {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Attribute Level
#[derive(Debug, Clone)]
pub struct Qualities_UpdateAttributeLevel {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Vital value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateAttribute2nd {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Vital value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateAttribute2nd {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Set or update a Character Vital value
#[derive(Debug, Clone)]
pub struct Qualities_PrivateUpdateAttribute2ndLevel {
    Sequence: String,
    Key: String,
    Value: String
}

// Set or update a Character Vital value
#[derive(Debug, Clone)]
pub struct Qualities_UpdateAttribute2ndLevel {
    Sequence: String,
    ObjectId: String,
    Key: String,
    Value: String
}

// Indirect '/e' text.
#[derive(Debug, Clone)]
pub struct Communication_HearEmote {
    SenderId: String,
    SenderName: String,
    Text: String
}

// Contains the text associated with an emote action.
#[derive(Debug, Clone)]
pub struct Communication_HearSoulEmote {
    SenderId: String,
    SenderName: String,
    Text: String
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Debug, Clone)]
pub struct Communication_HearSpeech {
    Message: String,
    SenderName: String,
    SenderId: String,
    Type: String
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Debug, Clone)]
pub struct Communication_HearRangedSpeech {
    Message: String,
    SenderName: String,
    SenderId: String,
    Range: String,
    Type: String
}

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Debug, Clone)]
pub struct Admin_Environs {
    EnvrionOption: String
}

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Debug, Clone)]
pub struct Movement_PositionAndMovementEvent {
    ObjectId: String,
    Position: String,
    MovementData: String
}

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Debug, Clone)]
pub struct Item_ObjDescEvent {
    ObjectId: String,
    ObjectDescription: String,
    InstanceSequence: String,
    VisualDescSequence: String
}

// Sets the player visual desc, TODO confirm this
#[derive(Debug, Clone)]
pub struct Character_SetPlayerVisualDesc {
    ObjectDescription: String
}

// Character creation screen initilised.
#[derive(Debug, Clone)]
pub struct Character_CharGenVerificationResponse {
    ResponseType: String,
    CharacterId: String,
    Name: String,
    SecondsUntilDeletion: String
}

// Sent when your subsciption is about to expire
#[derive(Debug, Clone)]
pub struct Login_AwaitingSubscriptionExpiration {
    SecondsRemaining: String
}

// The list of characters on the current account.
#[derive(Debug, Clone)]
pub struct Login_LoginCharacterSet {
    Status: String,
    Characters: String,
    DeletedCharacters: String,
    NumAllowedCharacters: String,
    Account: String,
    UseTurbineChat: String,
    HasThroneofDestiny: String
}

// Failure to log in
#[derive(Debug, Clone)]
pub struct Character_CharacterError {
    Reason: String
}

// Create an object somewhere in the world
#[derive(Debug, Clone)]
pub struct Item_CreateObject {
    ObjectId: String,
    ObjectDescription: String,
    PhysicsDescription: String,
    WeenieDescription: String
}

// Login of player
#[derive(Debug, Clone)]
pub struct Login_CreatePlayer {
    CharacterId: String
}

// Sent whenever an object is being deleted from the scene.
#[derive(Debug, Clone)]
pub struct Item_DeleteObject {
    ObjectId: String,
    ObjectInstanceSequence: String
}

// Sets the position/motion of an object
#[derive(Debug, Clone)]
pub struct Movement_PositionEvent {
    ObjectId: String,
    Position: String
}

// Sets the parent for an object, eg. equipting an object.
#[derive(Debug, Clone)]
pub struct Item_ParentEvent {
    ParentId: String,
    ChildId: String,
    Location: String,
    Placement: String,
    ObjectInstanceSequence: String,
    ChildPositionSequence: String
}

// Sent when picking up an object
#[derive(Debug, Clone)]
pub struct Inventory_PickupEvent {
    ObjectId: String,
    ObjectInstanceSequence: String,
    ObjectPositionSequence: String
}

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Debug, Clone)]
pub struct Item_SetState {
    ObjectId: String,
    NewState: String,
    ObjectInstanceSequence: String,
    ObjectStateSequence: String
}

// These are animations. Whenever a human, monster or object moves - one of these little messages is sent. Even idle emotes (like head scratching and nodding) are sent in this manner.
#[derive(Debug, Clone)]
pub struct Movement_SetObjectMovement {
    ObjectId: String,
    ObjectInstanceSequence: String,
    MovementData: String
}

// Changes an objects vector, for things like jumping
#[derive(Debug, Clone)]
pub struct Movement_VectorUpdate {
    ObjectId: String,
    Velocity: String,
    Omega: String,
    ObjectInstanceSequence: String,
    ObjectVectorSequence: String
}

// Applies a sound effect.
#[derive(Debug, Clone)]
pub struct Effects_SoundEvent {
    ObjectId: String,
    SoundType: String,
    Volume: String
}

// Instructs the client to show the portal graphic.
#[derive(Debug, Clone)]
pub struct Effects_PlayerTeleport {
    ObjectTeleportSequence: String
}

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Debug, Clone)]
pub struct Effects_PlayScriptId {
    ObjectId: String,
    ScriptId: String
}

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Debug, Clone)]
pub struct Effects_PlayScriptType {
    ObjectId: String,
    ScriptType: String,
    Speed: String
}

// Account has been banned
#[derive(Debug, Clone)]
pub struct Login_AccountBanned {
    BannedUntil: String,
    Text: String
}

// Admin Receive Account Data
#[derive(Debug, Clone)]
pub struct Admin_ReceiveAccountData {
    Unknown: String,
    AdminAccountData: String
}

// Admin Receive Player Data
#[derive(Debug, Clone)]
pub struct Admin_ReceivePlayerData {
    Unknown: String,
    AdminPlayerData: String
}

// Update an existing object's data.
#[derive(Debug, Clone)]
pub struct Item_UpdateObject {
    ObjectId: String,
    ObjectDesc: String,
    PhysicsDesc: String,
    WeenieDesc: String
}

// Account has been booted
#[derive(Debug, Clone)]
pub struct Login_AccountBooted {
    AdditionalReasonText: String,
    ReasonText: String
}

// Send or receive a message using Turbine Chat.
#[derive(Debug, Clone)]
pub struct Communication_TurbineChat {
    MessageSize: String,
    Type: String,
    BlobDispatchType: String,
    TargetType: String,
    TargetId: String,
    TransportType: String,
    TransportId: String,
    Cookie: String,
    PayloadSize: String,
    RoomId: String,
    DisplayName: String,
    Text: String,
    ExtraDataSize: String,
    SpeakerId: String,
    HResult: String,
    ChatType: String,
    ContextId: String,
    ResponseId: String,
    MethodId: String,
    RoomId: String,
    Text: String,
    ExtraDataSize: String,
    SpeakerId: String,
    HResult: String,
    ChatType: String,
    ContextId: String,
    ResponseId: String,
    MethodId: String,
    HResult: String,
    ContextId: String,
    ResponseId: String,
    MethodId: String,
    HResult: String
}

// Display a message in the chat window.
#[derive(Debug, Clone)]
pub struct Communication_TextboxString {
    Text: String,
    Type: String
}

// The name of the current world.
#[derive(Debug, Clone)]
pub struct Login_WorldInfo {
    Connections: String,
    MaxConnections: String,
    WorldName: String
}

// Add or update a dat file Resource.
#[derive(Debug, Clone)]
pub struct DDD_DataMessage {
    DatFile: String,
    ResourceType: String,
    ResourceId: String,
    Iteration: String,
    Compression: String,
    Version: String,
    DataSize: String,
    FileSize: String
}

// DDD error
#[derive(Debug, Clone)]
pub struct DDD_ErrorMessage {
    ResourceType: String,
    ResourceId: String,
    RError: String
}

// A list of dat files that need to be patched
#[derive(Debug, Clone)]
pub struct DDD_BeginDDDMessage {
    DataExpected: String,
    Revisions: String
}

// Add or update a dat file Resource.
#[derive(Debug, Clone)]
pub struct DDD_InterrogationMessage {
    ServersRegion: String,
    NameRuleLanguage: String,
    ProductId: String,
    SupportedLanguages: String
}

// Client to Server AC packet.
#[derive(Debug, Clone)]
pub struct C2SPacket {
    Sequence: String,
    Flags: String,
    Checksum: String,
    RecipientId: String,
    TimeSinceLastPacket: String,
    Size: String,
    Iteration: String,
    ServerSwitch: String,
    RetransmitSequences: String,
    RejectSequences: String,
    AckSequence: String,
    LoginRequest: String,
    WorldLoginRequest: String,
    ConnectResponse: String,
    CICMDCommand: String,
    Time: String,
    EchoTime: String,
    Flow: String,
    Fragments: String
}

// Server to Client AC packet.
#[derive(Debug, Clone)]
pub struct S2CPacket {
    Sequence: String,
    Flags: String,
    Checksum: String,
    RecipientId: String,
    TimeSinceLastPacket: String,
    Size: String,
    Iteration: String,
    AckSequence: String,
    LogonServerAddr: String,
    Referral: String,
    ConnectRequest: String,
    NetError: String,
    NetErrorDisconnect: String,
    EchoResponse: String,
    Fragments: String
}

