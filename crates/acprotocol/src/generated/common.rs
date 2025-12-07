use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketHeaderFlags {
        pub bits: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FragmentGroup {
    Event = 5,
    Private = 9,
    Object = 10,
}

/// The type of server to switch
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ServerSwitchType {
    World = 0,
    Logon = 1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AuthFlags {
    None = 0x0,
    EnableCrypto = 0x1,
    AdminAccountOverride = 0x2,
    LastDefault = 0x4,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum NetAuthType {
    Undef = 0x00000000,
    Account = 0x00000001,
    AccountPassword = 0x00000002,
    GlsTicket = 0x40000002,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameMessageGroup {
    Event = 0x0001,
    Control = 0x0002,
    Weenie = 0x0003,
    Logon = 0x0004,
    Database = 0x0005,
    SecureControl = 0x0006,
    SecureWeenie = 0x0007,
    SecureLogin = 0x0008,
    UIQueue = 0x0009,
    SmartBox = 0x000A,
    Observer = 0x0008,
}

/// Client to Server message opcodes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum C2SMessage {
    Login_LogOffCharacter = 0xF653,
    Character_CharacterDelete = 0xF655,
    Character_SendCharGenResult = 0xF656,
    Login_SendEnterWorld = 0xF657,
    Object_SendForceObjdesc = 0xF6EA,
    Login_SendEnterWorldRequest = 0xF7C8,
    Admin_SendAdminGetServerVersion = 0xF7CC,
    Social_SendFriendsCommand = 0xF7CD,
    Admin_SendAdminRestoreCharacter = 0xF7D9,
    Communication_TurbineChat = 0xF7DE,
    DDD_RequestDataMessage = 0xF7E3,
    DDD_InterrogationResponseMessage = 0xF7E6,
    DDD_OnEndDDD = 0xF7EA,
    DDD_EndDDDMessage = 0xF7EB,
    Ordered_GameAction = 0xF7B1,
}

/// Server to Client message opcodes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum S2CMessage {
    Item_ServerSaysRemove = 0x0024,
    Character_ServerSaysAttemptFailed = 0x00A0,
    Item_UpdateStackSize = 0x0197,
    Combat_HandlePlayerDeathEvent = 0x019E,
    Qualities_PrivateRemoveIntEvent = 0x01D1,
    Qualities_RemoveIntEvent = 0x01D2,
    Qualities_PrivateRemoveBoolEvent = 0x01D3,
    Qualities_RemoveBoolEvent = 0x01D4,
    Qualities_PrivateRemoveFloatEvent = 0x01D5,
    Qualities_RemoveFloatEvent = 0x01D6,
    Qualities_PrivateRemoveStringEvent = 0x01D7,
    Qualities_RemoveStringEvent = 0x01D8,
    Qualities_PrivateRemoveDataIdEvent = 0x01D9,
    Qualities_RemoveDataIdEvent = 0x01DA,
    Qualities_PrivateRemoveInstanceIdEvent = 0x01DB,
    Qualities_RemoveInstanceIdEvent = 0x01DC,
    Qualities_PrivateRemovePositionEvent = 0x01DD,
    Qualities_RemovePositionEvent = 0x01DE,
    Communication_HearEmote = 0x01E0,
    Communication_HearSoulEmote = 0x01E2,
    Qualities_PrivateRemoveInt64Event = 0x02B8,
    Qualities_RemoveInt64Event = 0x02B9,
    Communication_HearSpeech = 0x02BB,
    Communication_HearRangedSpeech = 0x02BC,
    Qualities_PrivateUpdateInt = 0x02CD,
    Qualities_UpdateInt = 0x02CE,
    Qualities_PrivateUpdateInt64 = 0x02CF,
    Qualities_UpdateInt64 = 0x02D0,
    Qualities_PrivateUpdateBool = 0x02D1,
    Qualities_UpdateBool = 0x02D2,
    Qualities_PrivateUpdateFloat = 0x02D3,
    Qualities_UpdateFloat = 0x02D4,
    Qualities_PrivateUpdateString = 0x02D5,
    Qualities_UpdateString = 0x02D6,
    Qualities_PrivateUpdateDataId = 0x02D7,
    Qualities_UpdateDataId = 0x02D8,
    Qualities_PrivateUpdateInstanceId = 0x02D9,
    Qualities_UpdateInstanceId = 0x02DA,
    Qualities_PrivateUpdatePosition = 0x02DB,
    Qualities_UpdatePosition = 0x02DC,
    Qualities_PrivateUpdateSkill = 0x02DD,
    Qualities_UpdateSkill = 0x02DE,
    Qualities_PrivateUpdateSkillLevel = 0x02DF,
    Qualities_UpdateSkillLevel = 0x02E0,
    Qualities_PrivateUpdateSkillAC = 0x02E1,
    Qualities_UpdateSkillAC = 0x02E2,
    Qualities_PrivateUpdateAttribute = 0x02E3,
    Qualities_UpdateAttribute = 0x02E4,
    Qualities_PrivateUpdateAttributeLevel = 0x02E5,
    Qualities_UpdateAttributeLevel = 0x02E6,
    Qualities_PrivateUpdateAttribute2nd = 0x02E7,
    Qualities_UpdateAttribute2nd = 0x02E8,
    Qualities_PrivateUpdateAttribute2ndLevel = 0x02E9,
    Qualities_UpdateAttribute2ndLevel = 0x02EA,
    Admin_Environs = 0xEA60,
    Movement_PositionAndMovementEvent = 0xF619,
    Item_ObjDescEvent = 0xF625,
    Character_SetPlayerVisualDesc = 0xF630,
    Character_CharGenVerificationResponse = 0xF643,
    Login_AwaitingSubscriptionExpiration = 0xF651,
    Login_LogOffCharacter = 0xF653,
    Character_CharacterDelete = 0xF655,
    Login_LoginCharacterSet = 0xF658,
    Character_CharacterError = 0xF659,
    Item_CreateObject = 0xF745,
    Login_CreatePlayer = 0xF746,
    Item_DeleteObject = 0xF747,
    Movement_PositionEvent = 0xF748,
    Item_ParentEvent = 0xF749,
    Inventory_PickupEvent = 0xF74A,
    Item_SetState = 0xF74B,
    Movement_SetObjectMovement = 0xF74C,
    Movement_VectorUpdate = 0xF74E,
    Effects_SoundEvent = 0xF750,
    Effects_PlayerTeleport = 0xF751,
    Effects_PlayScriptId = 0xF754,
    Effects_PlayScriptType = 0xF755,
    Login_AccountBanned = 0xF7C1,
    Admin_ReceiveAccountData = 0xF7CA,
    Admin_ReceivePlayerData = 0xF7CB,
    Item_UpdateObject = 0xF7DB,
    Login_AccountBooted = 0xF7DC,
    Communication_TurbineChat = 0xF7DE,
    Login_EnterGame_ServerReady = 0xF7DF,
    Communication_TextboxString = 0xF7E0,
    Login_WorldInfo = 0xF7E1,
    DDD_DataMessage = 0xF7E2,
    DDD_ErrorMessage = 0xF7E4,
    DDD_InterrogationMessage = 0xF7E5,
    DDD_BeginDDDMessage = 0xF7E7,
    DDD_OnEndDDD = 0xF7EA,
    Ordered_GameEvent = 0xF7B0,
}

/// Ordered (0xF7B0) Server to Client opcodes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    Allegiance_AllegianceUpdateAborted = 0x0003,
    Communication_PopUpString = 0x0004,
    Login_PlayerDescription = 0x0013,
    Allegiance_AllegianceUpdate = 0x0020,
    Social_FriendsUpdate = 0x0021,
    Item_ServerSaysContainId = 0x0022,
    Item_WearItem = 0x0023,
    Social_CharacterTitleTable = 0x0029,
    Social_AddOrSetCharacterTitle = 0x002B,
    Item_StopViewingObjectContents = 0x0052,
    Vendor_VendorInfo = 0x0062,
    Character_StartBarber = 0x0075,
    Fellowship_Quit = 0x00A3,
    Fellowship_Dismiss = 0x00A4,
    Writing_BookOpen = 0x00B4,
    Writing_BookAddPageResponse = 0x00B6,
    Writing_BookDeletePageResponse = 0x00B7,
    Writing_BookPageDataResponse = 0x00B8,
    Item_GetInscriptionResponse = 0x00C3,
    Item_SetAppraiseInfo = 0x00C9,
    Communication_ChannelBroadcast = 0x0147,
    Communication_ChannelList = 0x0148,
    Communication_ChannelIndex = 0x0149,
    Item_OnViewContents = 0x0196,
    Item_ServerSaysMoveItem = 0x019A,
    Combat_HandleAttackDoneEvent = 0x01A7,
    Magic_RemoveSpell = 0x01A8,
    Combat_HandleVictimNotificationEventSelf = 0x01AC,
    Combat_HandleVictimNotificationEventOther = 0x01AD,
    Combat_HandleAttackerNotificationEvent = 0x01B1,
    Combat_HandleDefenderNotificationEvent = 0x01B2,
    Combat_HandleEvasionAttackerNotificationEvent = 0x01B3,
    Combat_HandleEvasionDefenderNotificationEvent = 0x01B4,
    Combat_HandleCommenceAttackEvent = 0x01B8,
    Combat_QueryHealthResponse = 0x01C0,
    Character_QueryAgeResponse = 0x01C3,
    Item_UseDone = 0x01C7,
    Allegiance_AllegianceUpdateDone = 0x01C8,
    Fellowship_FellowUpdateDone = 0x01C9,
    Fellowship_FellowStatsDone = 0x01CA,
    Item_AppraiseDone = 0x01CB,
    Character_ReturnPing = 0x01EA,
    Communication_SetSquelchDB = 0x01F4,
    Trade_RegisterTrade = 0x01FD,
    Trade_OpenTrade = 0x01FE,
    Trade_CloseTrade = 0x01FF,
    Trade_AddToTrade = 0x0200,
    Trade_RemoveFromTrade = 0x0201,
    Trade_AcceptTrade = 0x0202,
    Trade_DeclineTrade = 0x0203,
    Trade_ResetTrade = 0x0205,
    Trade_TradeFailure = 0x0207,
    Trade_ClearTradeAcceptance = 0x0208,
    House_HouseProfile = 0x021D,
    House_HouseData = 0x0225,
    House_HouseStatus = 0x0226,
    House_UpdateRentTime = 0x0227,
    House_UpdateRentPayment = 0x0228,
    House_UpdateRestrictions = 0x0248,
    House_UpdateHAR = 0x0257,
    House_HouseTransaction = 0x0259,
    Item_QueryItemManaResponse = 0x0264,
    House_AvailableHouses = 0x0271,
    Character_ConfirmationRequest = 0x0274,
    Character_ConfirmationDone = 0x0276,
    Allegiance_AllegianceLoginNotificationEvent = 0x027A,
    Allegiance_AllegianceInfoResponseEvent = 0x027C,
    Game_JoinGameResponse = 0x0281,
    Game_StartGame = 0x0282,
    Game_MoveResponse = 0x0283,
    Game_OpponentTurn = 0x0284,
    Game_OpponentStalemateState = 0x0285,
    Communication_WeenieError = 0x028A,
    Communication_WeenieErrorWithString = 0x028B,
    Game_GameOver = 0x028C,
    Communication_ChatRoomTracker = 0x0295,
    Admin_QueryPluginList = 0x02AE,
    Admin_QueryPlugin = 0x02B1,
    Admin_QueryPluginResponse2 = 0x02B3,
    Inventory_SalvageOperationsResultData = 0x02B4,
    Communication_HearDirectSpeech = 0x02BD,
    Fellowship_FullUpdate = 0x02BE,
    Fellowship_Disband = 0x02BF,
    Fellowship_UpdateFellow = 0x02C0,
    Magic_UpdateSpell = 0x02C1,
    Magic_UpdateEnchantment = 0x02C2,
    Magic_RemoveEnchantment = 0x02C3,
    Magic_UpdateMultipleEnchantments = 0x02C4,
    Magic_RemoveMultipleEnchantments = 0x02C5,
    Magic_PurgeEnchantments = 0x02C6,
    Magic_DispelEnchantment = 0x02C7,
    Magic_DispelMultipleEnchantments = 0x02C8,
    Misc_PortalStormBrewing = 0x02C9,
    Misc_PortalStormImminent = 0x02CA,
    Misc_PortalStorm = 0x02CB,
    Misc_PortalStormSubsided = 0x02CC,
    Communication_TransientString = 0x02EB,
    Magic_PurgeBadEnchantments = 0x0312,
    Social_SendClientContractTrackerTable = 0x0314,
    Social_SendClientContractTracker = 0x0315,
}

/// Ordered (0xF7B1) Client to server opcodes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GameAction {
    Character_PlayerOptionChangedEvent = 0x0005,
    Combat_TargetedMeleeAttack = 0x0008,
    Combat_TargetedMissileAttack = 0x000A,
    Communication_SetAFKMode = 0x000F,
    Communication_SetAFKMessage = 0x0010,
    Communication_Talk = 0x0015,
    Social_RemoveFriend = 0x0017,
    Social_AddFriend = 0x0018,
    Inventory_PutItemInContainer = 0x0019,
    Inventory_GetAndWieldItem = 0x001A,
    Inventory_DropItem = 0x001B,
    Allegiance_SwearAllegiance = 0x001D,
    Allegiance_BreakAllegiance = 0x001E,
    Allegiance_UpdateRequest = 0x001F,
    Social_ClearFriends = 0x0025,
    Character_TeleToPKLArena = 0x0026,
    Character_TeleToPKArena = 0x0027,
    Social_SetDisplayCharacterTitle = 0x002C,
    Allegiance_QueryAllegianceName = 0x0030,
    Allegiance_ClearAllegianceName = 0x0031,
    Communication_TalkDirect = 0x0032,
    Allegiance_SetAllegianceName = 0x0033,
    Inventory_UseWithTargetEvent = 0x0035,
    Inventory_UseEvent = 0x0036,
    Allegiance_SetAllegianceOfficer = 0x003B,
    Allegiance_SetAllegianceOfficerTitle = 0x003C,
    Allegiance_ListAllegianceOfficerTitles = 0x003D,
    Allegiance_ClearAllegianceOfficerTitles = 0x003E,
    Allegiance_DoAllegianceLockAction = 0x003F,
    Allegiance_SetAllegianceApprovedVassal = 0x0040,
    Allegiance_AllegianceChatGag = 0x0041,
    Allegiance_DoAllegianceHouseAction = 0x0042,
    Train_TrainAttribute2nd = 0x0044,
    Train_TrainAttribute = 0x0045,
    Train_TrainSkill = 0x0046,
    Train_TrainSkillAdvancementClass = 0x0047,
    Magic_CastUntargetedSpell = 0x0048,
    Magic_CastTargetedSpell = 0x004A,
    Combat_ChangeCombatMode = 0x0053,
    Inventory_StackableMerge = 0x0054,
    Inventory_StackableSplitToContainer = 0x0055,
    Inventory_StackableSplitTo3D = 0x0056,
    Communication_ModifyCharacterSquelch = 0x0058,
    Communication_ModifyAccountSquelch = 0x0059,
    Communication_ModifyGlobalSquelch = 0x005B,
    Communication_TalkDirectByName = 0x005D,
    Vendor_Buy = 0x005F,
    Vendor_Sell = 0x0060,
    Character_TeleToLifestone = 0x0063,
    Character_LoginCompleteNotification = 0x00A1,
    Fellowship_Create = 0x00A2,
    Fellowship_Quit = 0x00A3,
    Fellowship_Dismiss = 0x00A4,
    Fellowship_Recruit = 0x00A5,
    Fellowship_UpdateRequest = 0x00A6,
    Writing_BookAddPage = 0x00AA,
    Writing_BookModifyPage = 0x00AB,
    Writing_BookData = 0x00AC,
    Writing_BookDeletePage = 0x00AD,
    Writing_BookPageData = 0x00AE,
    Writing_SetInscription = 0x00BF,
    Item_Appraise = 0x00C8,
    Inventory_GiveObjectRequest = 0x00CD,
    Advocate_Teleport = 0x00D6,
    Character_AbuseLogRequest = 0x0140,
    Communication_AddToChannel = 0x0145,
    Communication_RemoveFromChannel = 0x0146,
    Communication_ChannelBroadcast = 0x0147,
    Communication_ChannelList = 0x0148,
    Communication_ChannelIndex = 0x0149,
    Inventory_NoLongerViewingContents = 0x0195,
    Inventory_StackableSplitToWield = 0x019B,
    Character_AddShortCut = 0x019C,
    Character_RemoveShortCut = 0x019D,
    Character_CharacterOptionsEvent = 0x01A1,
    Magic_RemoveSpell = 0x01A8,
    Combat_CancelAttack = 0x01B7,
    Combat_QueryHealth = 0x01BF,
    Character_QueryAge = 0x01C2,
    Character_QueryBirth = 0x01C4,
    Communication_Emote = 0x01DF,
    Communication_SoulEmote = 0x01E1,
    Character_AddSpellFavorite = 0x01E3,
    Character_RemoveSpellFavorite = 0x01E4,
    Character_RequestPing = 0x01E9,
    Trade_OpenTradeNegotiations = 0x01F6,
    Trade_CloseTradeNegotiations = 0x01F7,
    Trade_AddToTrade = 0x01F8,
    Trade_AcceptTrade = 0x01FA,
    Trade_DeclineTrade = 0x01FB,
    Trade_ResetTrade = 0x0204,
    Character_ClearPlayerConsentList = 0x0216,
    Character_DisplayPlayerConsentList = 0x0217,
    Character_RemoveFromPlayerConsentList = 0x0218,
    Character_AddPlayerPermission = 0x0219,
    House_BuyHouse = 0x021C,
    House_QueryHouse = 0x021E,
    House_AbandonHouse = 0x021F,
    Character_RemovePlayerPermission = 0x0220,
    House_RentHouse = 0x0221,
    Character_SetDesiredComponentLevel = 0x0224,
    House_AddPermanentGuest = 0x0245,
    House_RemovePermanentGuest = 0x0246,
    House_SetOpenHouseStatus = 0x0247,
    House_ChangeStoragePermission = 0x0249,
    House_BootSpecificHouseGuest = 0x024A,
    House_RemoveAllStoragePermission = 0x024C,
    House_RequestFullGuestList = 0x024D,
    Allegiance_SetMotd = 0x0254,
    Allegiance_QueryMotd = 0x0255,
    Allegiance_ClearMotd = 0x0256,
    House_QueryLord = 0x0258,
    House_AddAllStoragePermission = 0x025C,
    House_RemoveAllPermanentGuests = 0x025E,
    House_BootEveryone = 0x025F,
    House_TeleToHouse = 0x0262,
    Item_QueryItemMana = 0x0263,
    House_SetHooksVisibility = 0x0266,
    House_ModifyAllegianceGuestPermission = 0x0267,
    House_ModifyAllegianceStoragePermission = 0x0268,
    Game_Join = 0x0269,
    Game_Quit = 0x026A,
    Game_Move = 0x026B,
    Game_MovePass = 0x026D,
    Game_Stalemate = 0x026E,
    House_ListAvailableHouses = 0x0270,
    Character_ConfirmationResponse = 0x0275,
    Allegiance_BreakAllegianceBoot = 0x0277,
    House_TeleToMansion = 0x0278,
    Character_Suicide = 0x0279,
    Allegiance_AllegianceInfoRequest = 0x027B,
    Inventory_CreateTinkeringTool = 0x027D,
    Character_SpellbookFilterEvent = 0x0286,
    Character_TeleToMarketplace = 0x028D,
    Character_EnterPKLite = 0x028F,
    Fellowship_AssignNewLeader = 0x0290,
    Fellowship_ChangeFellowOpeness = 0x0291,
    Allegiance_AllegianceChatBoot = 0x02A0,
    Allegiance_AddAllegianceBan = 0x02A1,
    Allegiance_RemoveAllegianceBan = 0x02A2,
    Allegiance_ListAllegianceBans = 0x02A3,
    Allegiance_RemoveAllegianceOfficer = 0x02A5,
    Allegiance_ListAllegianceOfficers = 0x02A6,
    Allegiance_ClearAllegianceOfficers = 0x02A7,
    Allegiance_RecallAllegianceHometown = 0x02AB,
    Admin_QueryPluginListResponse = 0x02AF,
    Admin_QueryPluginResponse = 0x02B2,
    Character_FinishBarber = 0x0311,
    Social_AbandonContract = 0x0316,
    Movement_Jump = 0xF61B,
    Movement_MoveToState = 0xF61C,
    Movement_DoMovementCommand = 0xF61E,
    Movement_StopMovementCommand = 0xF661,
    Movement_AutonomyLevel = 0xF752,
    Movement_AutonomousPosition = 0xF753,
    Movement_Jump_NonAutonomous = 0xF7C9,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WeenieType {
    Undef = 0,
    Generic = 1,
    Clothing = 2,
    MissileLauncher = 3,
    Missile = 4,
    Ammunition = 5,
    MeleeWeapon = 6,
    Portal = 7,
    Book = 8,
    Coin = 9,
    Creature = 10,
    Admin = 11,
    Vendor = 12,
    HotSpot = 13,
    Corpse = 14,
    Cow = 15,
    AI = 16,
    Machine = 17,
    Food = 18,
    Door = 19,
    Chest = 20,
    Container = 21,
    Key = 22,
    Lockpick = 23,
    PressurePlate = 24,
    LifeStone = 25,
    Switch = 26,
    PKModifier = 27,
    Healer = 28,
    LightSource = 29,
    Allegiance = 30,
    UNKNOWN__GUESSEDNAME32 = 31,
    SpellComponent = 32,
    ProjectileSpell = 33,
    Scroll = 34,
    Caster = 35,
    Channel = 36,
    ManaStone = 37,
    Gem = 38,
    AdvocateFane = 39,
    AdvocateItem = 40,
    Sentinel = 41,
    GSpellEconomy = 42,
    LSpellEconomy = 43,
    CraftTool = 44,
    LScoreKeeper = 45,
    GScoreKeeper = 46,
    GScoreGatherer = 47,
    ScoreBook = 48,
    EventCoordinator = 49,
    Entity = 50,
    Stackable = 51,
    HUD = 52,
    House = 53,
    Deed = 54,
    SlumLord = 55,
    Hook = 56,
    Storage = 57,
    BootSpot = 58,
    HousePortal = 59,
    Game = 60,
    GamePiece = 61,
    SkillAlterationDevice = 62,
    AttributeTransferDevice = 63,
    Hooker = 64,
    AllegianceBindstone = 65,
    InGameStatKeeper = 66,
    AugmentationDevice = 67,
    SocialManager = 68,
    Pet = 69,
    PetDevice = 70,
    CombatPet = 71,
}

/// Flags that dictate what property tables are included with the ACBaseQuali
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACBaseQualitiesFlags {
        pub bits: u32,
}

/// Set of predefined error messages that accept interpolated string argument
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WeenieErrorWithString {
    IsTooBusyToAcceptGifts = 0x001E,
    CannotCarryAnymore = 0x002B,
    YouFailToAffect_YouCannotAffectAnyone = 0x004E,
    YouFailToAffect_TheyCannotBeHarmed = 0x004F,
    YouFailToAffect_WithBeneficialSpells = 0x0050,
    YouFailToAffect_YouAreNotPK = 0x0051,
    YouFailToAffect_TheyAreNotPK = 0x0052,
    YouFailToAffect_NotSamePKType = 0x0053,
    YouFailToAffect_AcrossHouseBoundary = 0x0054,
    IsNotAcceptingGiftsRightNow = 0x03EF,
    IsAlreadyOneOfYourFollowers = 0x0413,
    CannotHaveAnyMoreVassals = 0x0416,
    DoesntKnowWhatToDoWithThat = 0x046A,
    YouMustBeAboveLevel_ToBuyHouse = 0x0488,
    YouMustBeAtOrBelowLevel_ToBuyHouse = 0x0489,
    YouMustBeAboveAllegianceRank_ToBuyHouse = 0x048B,
    YouMustBeAtOrBelowAllegianceRank_ToBuyHouse = 0x048C,
    The_WasNotSuitableForSalvaging = 0x04BF,
    The_ContainseTheWrongMaterial = 0x04C0,
    YouMustBe_ToUseItemMagic = 0x04C6,
    Your_IsTooLowToUseItemMagic = 0x04C9,
    Only_MayUseItemMagic = 0x04CA,
    YouMustSpecialize_ToUseItemMagic = 0x04CB,
    AiRefuseItemDuringEmote = 0x04CE,
    CannotAcceptStackedItems = 0x04CF,
    Your_SkillMustBeTrained = 0x04D1,
    NotEnoughSkillCreditsToSpecialize = 0x04D2,
    TooMuchXPToRecoverFromSkill = 0x04D3,
    Your_SkillIsAlreadyUntrained = 0x04D4,
    CannotLowerSkillWhileWieldingItem = 0x04D5,
    YouHaveSucceededSpecializing_Skill = 0x04D6,
    YouHaveSucceededUnspecializing_Skill = 0x04D7,
    YouHaveSucceededUntraining_Skill = 0x04D8,
    CannotUntrain_SkillButRecoveredXP = 0x04D9,
    TooManyCreditsInSpecializedSkills = 0x04DA,
    AttributeTransferFromTooLow = 0x04DE,
    AttributeTransferToTooHigh = 0x04DF,
    ItemUnusableOnHook_CannotOpen = 0x04E8,
    ItemUnusableOnHook_CanOpen = 0x04E9,
    ItemOnlyUsableOnHook = 0x04EA,
    FailsToAffectYou_TheyCannotAffectAnyone = 0x04F4,
    FailsToAffectYou_YouCannotBeHarmed = 0x04F5,
    FailsToAffectYou_TheyAreNotPK = 0x04F6,
    FailsToAffectYou_YouAreNotPK = 0x04F7,
    FailsToAffectYou_NotSamePKType = 0x04F8,
    FailsToAffectYouAcrossHouseBoundary = 0x04F9,
    IsAnInvalidTarget = 0x04FA,
    YouAreInvalidTargetForSpellOf_ = 0x04FB,
    IsAtFullHealth = 0x04FF,
    HasNoSpellTargets = 0x0509,
    YouHaveNoTargetsForSpellOf_ = 0x050A,
    IsNowOpenFellowship = 0x050B,
    IsNowClosedFellowship = 0x050C,
    IsNowLeaderOfFellowship = 0x050D,
    YouHavePassedFellowshipLeadershipTo_ = 0x050E,
    MaxNumberOf_Hooked = 0x0510,
    MaxNumberOf_HookedUntilOneIsRemoved = 0x0514,
    NoLongerMaxNumberOf_Hooked = 0x0515,
    IsNotCloseEnoughToYourLevel = 0x0517,
    LockedFellowshipCannotRecruit_ = 0x0518,
    YouHaveEnteredThe_Channel = 0x051B,
    YouHaveLeftThe_Channel = 0x051C,
    WillNotReceiveMessage = 0x051E,
    MessageBlocked_ = 0x051F,
    HasBeenAddedToHearList = 0x0521,
    HasBeenRemovedFromHearList = 0x0522,
    FailToRemove_FromLoudList = 0x0525,
    YouCannotOpenLockedFellowship = 0x0528,
    YouAreNowSnoopingOn_ = 0x052C,
    YouAreNoLongerSnoopingOn_ = 0x052D,
    YouFailToSnoopOn_ = 0x052E,
    AttemptedToSnoopOnYou = 0x052F,
    IsAlreadyBeingSnoopedOn = 0x0530,
    IsInLimbo = 0x0531,
    YouHaveBeenBootedFromAllegianceChat = 0x0533,
    HasBeenBootedFromAllegianceChat = 0x0534,
    AccountOf_IsAlreadyBannedFromAllegiance = 0x0536,
    AccountOf_IsNotBannedFromAllegiance = 0x0537,
    AccountOf_WasNotUnbannedFromAllegiance = 0x0538,
    AccountOf_IsBannedFromAllegiance = 0x0539,
    AccountOf_IsUnbannedFromAllegiance = 0x053A,
    ListOfBannedCharacters = 0x053B,
    IsBannedFromAllegiance = 0x053E,
    YouAreBannedFromAllegiance = 0x053F,
    IsNowAllegianceOfficer = 0x0541,
    ErrorSetting_AsAllegianceOfficer = 0x0542,
    IsNoLongerAllegianceOfficer = 0x0543,
    ErrorRemoving_AsAllegianceOFficer = 0x0544,
    YouMustWait_BeforeCommunicating = 0x0547,
    YourAllegianceOfficerStatusChanged = 0x0549,
    IsAlreadyAllegianceOfficerOfThatLevel = 0x054B,
    The_IsCurrentlyInUse = 0x54D,
    YouAreNotListeningTo_Channel = 0x0551,
    AugmentationSkillNotTrained = 0x055A,
    YouSuccededAcquiringAugmentation = 0x055B,
    YouSucceededRecoveringXPFromSkill_AugmentationNotUntrainable = 0x055C,
    AFK = 0x055E,
    IsAlreadyOnYourFriendsList = 0x0562,
    YouMayOnlyChangeAllegianceNameOnceEvery24Hours = 0x056A,
    IsTheMonarchAndCannotBePromotedOrDemoted = 0x056D,
    ThatLevelOfAllegianceOfficerIsNowKnownAs_ = 0x056E,
    YourAllegianceIsCurrently_ = 0x0574,
    YourAllegianceIsNow_ = 0x0575,
    YouCannotAcceptAllegiance_YourAllegianceIsLocked = 0x0576,
    YouCannotSwearAllegiance_AllegianceOf_IsLocked = 0x0577,
    YouHavePreApproved_ToJoinAllegiance = 0x0578,
    IsAlreadyMemberOfYourAllegiance = 0x057A,
    HasBeenPreApprovedToJoinYourAllegiance = 0x057B,
    YourAllegianceChatPrivilegesRemoved = 0x057F,
    IsTemporarilyGaggedInAllegianceChat = 0x0580,
    YourAllegianceChatPrivilegesRestoredBy_ = 0x0582,
    YouRestoreAllegianceChatPrivilegesTo_ = 0x0583,
    CowersFromYou = 0x058A,
}

/// Set of predefined error messages
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WeenieError {
    None = 0x0000,
    NoMem = 0x0001,
    BadParam = 0x0002,
    DivZero = 0x0003,
    SegV = 0x0004,
    Unimplemented = 0x0005,
    UnknownMessageType = 0x0006,
    NoAnimationTable = 0x0007,
    NoPhysicsObject = 0x0008,
    NoBookieObject = 0x0009,
    NoWslObject = 0x000A,
    NoMotionInterpreter = 0x000B,
    UnhandledSwitch = 0x000C,
    DefaultConstructorCalled = 0x000D,
    InvalidCombatManeuver = 0x000E,
    BadCast = 0x000F,
    MissingQuality = 0x0010,
    MissingDatabaseObject = 0x0012,
    NoCallbackSet = 0x0013,
    CorruptQuality = 0x0014,
    BadContext = 0x0015,
    NoEphseqManager = 0x0016,
    BadMovementEvent = 0x0017,
    CannotCreateNewObject = 0x0018,
    NoControllerObject = 0x0019,
    CannotSendEvent = 0x001A,
    PhysicsCantTransition = 0x001B,
    PhysicsMaxDistanceExceeded = 0x001C,
    YoureTooBusy = 0x001D,
    CannotSendMessage = 0x001F,
    IllegalInventoryTransaction = 0x0020,
    ExternalWeenieObject = 0x0021,
    InternalWeenieObject = 0x0022,
    MotionFailure = 0x0023,
    YouCantJumpWhileInTheAir = 0x0024,
    InqCylSphereFailure = 0x0025,
    ThatIsNotAValidCommand = 0x0026,
    CarryingItem = 0x0027,
    Frozen = 0x0028,
    Stuck = 0x0029,
    YouAreTooEncumbered = 0x002A,
    BadContain = 0x002C,
    BadParent = 0x002D,
    BadDrop = 0x002E,
    BadRelease = 0x002F,
    MsgBadMsg = 0x0030,
    MsgUnpackFailed = 0x0031,
    MsgNoMsg = 0x0032,
    MsgUnderflow = 0x0033,
    MsgOverflow = 0x0034,
    MsgCallbackFailed = 0x0035,
    ActionCancelled = 0x0036,
    ObjectGone = 0x0037,
    NoObject = 0x0038,
    CantGetThere = 0x0039,
    Dead = 0x003A,
    ILeftTheWorld = 0x003B,
    ITeleported = 0x003C,
    YouChargedTooFar = 0x003D,
    YouAreTooTiredToDoThat = 0x003E,
    CantCrouchInCombat = 0x003F,
    CantSitInCombat = 0x0040,
    CantLieDownInCombat = 0x0041,
    CantChatEmoteInCombat = 0x0042,
    NoMtableData = 0x0043,
    CantChatEmoteNotStanding = 0x0044,
    TooManyActions = 0x0045,
    Hidden = 0x0046,
    GeneralMovementFailure = 0x0047,
    YouCantJumpFromThisPosition = 0x0048,
    CantJumpLoadedDown = 0x0049,
    YouKilledYourself = 0x004A,
    MsgResponseFailure = 0x004B,
    ObjectIsStatic = 0x004C,
    InvalidPkStatus = 0x004D,
    InvalidXpAmount = 0x03E9,
    InvalidPpCalculation = 0x03EA,
    InvalidCpCalculation = 0x03EB,
    UnhandledStatAnswer = 0x03EC,
    HeartAttack = 0x03ED,
    TheContainerIsClosed = 0x03EE,
    InvalidInventoryLocation = 0x03F0,
    ChangeCombatModeFailure = 0x03F1,
    FullInventoryLocation = 0x03F2,
    ConflictingInventoryLocation = 0x03F3,
    ItemNotPending = 0x03F4,
    BeWieldedFailure = 0x03F5,
    BeDroppedFailure = 0x03F6,
    YouAreTooFatiguedToAttack = 0x03F7,
    YouAreOutOfAmmunition = 0x03F8,
    YourAttackMisfired = 0x03F9,
    YouveAttemptedAnImpossibleSpellPath = 0x03FA,
    MagicIncompleteAnimList = 0x03FB,
    MagicInvalidSpellType = 0x03FC,
    MagicInqPositionAndVelocityFailure = 0x03FD,
    YouDontKnowThatSpell = 0x03FE,
    IncorrectTargetType = 0x03FF,
    YouDontHaveAllTheComponents = 0x0400,
    YouDontHaveEnoughManaToCast = 0x0401,
    YourSpellFizzled = 0x0402,
    YourSpellTargetIsMissing = 0x0403,
    YourProjectileSpellMislaunched = 0x0404,
    MagicSpellbookAddSpellFailure = 0x0405,
    MagicTargetOutOfRange = 0x0406,
    YourSpellCannotBeCastOutside = 0x0407,
    YourSpellCannotBeCastInside = 0x0408,
    MagicGeneralFailure = 0x0409,
    YouAreUnpreparedToCastASpell = 0x040A,
    YouveAlreadySwornAllegiance = 0x040B,
    CantSwearAllegianceInsufficientXp = 0x040C,
    AllegianceIgnoringRequests = 0x040D,
    AllegianceSquelched = 0x040E,
    AllegianceMaxDistanceExceeded = 0x040F,
    AllegianceIllegalLevel = 0x0410,
    AllegianceBadCreation = 0x0411,
    AllegiancePatronBusy = 0x0412,
    YouAreNotInAllegiance = 0x0414,
    AllegianceRemoveHierarchyFailure = 0x0415,
    FellowshipIgnoringRequests = 0x0417,
    FellowshipSquelched = 0x0418,
    FellowshipMaxDistanceExceeded = 0x0419,
    FellowshipMember = 0x041A,
    FellowshipIllegalLevel = 0x041B,
    FellowshipRecruitBusy = 0x041C,
    YouMustBeLeaderOfFellowship = 0x041D,
    YourFellowshipIsFull = 0x041E,
    FellowshipNameIsNotPermitted = 0x041F,
    LevelTooLow = 0x0420,
    LevelTooHigh = 0x0421,
    ThatChannelDoesntExist = 0x0422,
    YouCantUseThatChannel = 0x0423,
    YouAreAlreadyOnThatChannel = 0x0424,
    YouAreNotOnThatChannel = 0x0425,
    AttunedItem = 0x0426,
    YouCannotMergeDifferentStacks = 0x0427,
    YouCannotMergeEnchantedItems = 0x0428,
    YouMustControlAtLeastOneStack = 0x0429,
    CurrentlyAttacking = 0x042A,
    MissileAttackNotOk = 0x042B,
    TargetNotAcquired = 0x042C,
    ImpossibleShot = 0x042D,
    BadWeaponSkill = 0x042E,
    UnwieldFailure = 0x042F,
    LaunchFailure = 0x0430,
    ReloadFailure = 0x0431,
    UnableToMakeCraftReq = 0x0432,
    CraftAnimationFailed = 0x0433,
    YouCantCraftWithThatNumberOfItems = 0x0434,
    CraftGeneralErrorUiMsg = 0x0435,
    CraftGeneralErrorNoUiMsg = 0x0436,
    YouDoNotPassCraftingRequirements = 0x0437,
    YouDoNotHaveAllTheNecessaryItems = 0x0438,
    NotAllTheItemsAreAvailable = 0x0439,
    YouMustBeInPeaceModeToTrade = 0x043A,
    YouAreNotTrainedInThatTradeSkill = 0x043B,
    YourHandsMustBeFree = 0x043C,
    YouCannotLinkToThatPortal = 0x043D,
    YouHaveSolvedThisQuestTooRecently = 0x043E,
    YouHaveSolvedThisQuestTooManyTimes = 0x043F,
    QuestUnknown = 0x0440,
    QuestTableCorrupt = 0x0441,
    QuestBad = 0x0442,
    QuestDuplicate = 0x0443,
    QuestUnsolved = 0x0444,
    ItemRequiresQuestToBePickedUp = 0x0445,
    QuestSolvedTooLongAgo = 0x0446,
    TradeIgnoringRequests = 0x044C,
    TradeSquelched = 0x044D,
    TradeMaxDistanceExceeded = 0x044E,
    TradeAlreadyTrading = 0x044F,
    TradeBusy = 0x0450,
    TradeClosed = 0x0451,
    TradeExpired = 0x0452,
    TradeItemBeingTraded = 0x0453,
    TradeNonEmptyContainer = 0x0454,
    TradeNonCombatMode = 0x0455,
    TradeIncomplete = 0x0456,
    TradeStampMismatch = 0x0457,
    TradeUnopened = 0x0458,
    TradeEmpty = 0x0459,
    TradeAlreadyAccepted = 0x045A,
    TradeOutOfSync = 0x045B,
    PKsMayNotUsePortal = 0x045C,
    NonPKsMayNotUsePortal = 0x045D,
    HouseAbandoned = 0x045E,
    HouseEvicted = 0x045F,
    HouseAlreadyOwned = 0x0460,
    HouseBuyFailed = 0x0461,
    HouseRentFailed = 0x0462,
    Hooked = 0x0463,
    MagicInvalidPosition = 0x0465,
    YouMustHaveDarkMajestyToUsePortal = 0x0466,
    InvalidAmmoType = 0x0467,
    SkillTooLow = 0x0468,
    YouHaveUsedAllTheHooks = 0x0469,
    TradeAiDoesntWant = 0x046A,
    HookHouseNotOwned = 0x046B,
    YouMustCompleteQuestToUsePortal = 0x0474,
    HouseNoAllegiance = 0x047E,
    YouMustOwnHouseToUseCommand = 0x047F,
    YourMonarchDoesNotOwnAMansionOrVilla = 0x0480,
    YourMonarchsHouseIsNotAMansionOrVilla = 0x0481,
    YourMonarchHasClosedTheMansion = 0x0482,
    YouMustBeMonarchToPurchaseDwelling = 0x048A,
    AllegianceTimeout = 0x048D,
    YourOfferOfAllegianceWasIgnored = 0x048E,
    ConfirmationInProgress = 0x048F,
    YouMustBeAMonarchToUseCommand = 0x0490,
    YouMustSpecifyCharacterToBoot = 0x0491,
    YouCantBootYourself = 0x0492,
    ThatCharacterDoesNotExist = 0x0493,
    ThatPersonIsNotInYourAllegiance = 0x0494,
    CantBreakFromPatronNotInAllegiance = 0x0495,
    YourAllegianceHasBeenDissolved = 0x0496,
    YourPatronsAllegianceHasBeenBroken = 0x0497,
    YouHaveMovedTooFar = 0x0498,
    TeleToInvalidPosition = 0x0499,
    MustHaveDarkMajestyToUse = 0x049A,
    YouFailToLinkWithLifestone = 0x049B,
    YouWanderedTooFarToLinkWithLifestone = 0x049C,
    YouSuccessfullyLinkWithLifestone = 0x049D,
    YouMustLinkToLifestoneToRecall = 0x049E,
    YouFailToRecallToLifestone = 0x049F,
    YouFailToLinkWithPortal = 0x04A0,
    YouSuccessfullyLinkWithPortal = 0x04A1,
    YouFailToRecallToPortal = 0x04A2,
    YouMustLinkToPortalToRecall = 0x04A3,
    YouFailToSummonPortal = 0x04A4,
    YouMustLinkToPortalToSummonIt = 0x04A5,
    YouFailToTeleport = 0x04A6,
    YouHaveBeenTeleportedTooRecently = 0x04A7,
    YouMustBeAnAdvocateToUsePortal = 0x04A8,
    PortalAisNotAllowed = 0x04A9,
    PlayersMayNotUsePortal = 0x04AA,
    YouAreNotPowerfulEnoughToUsePortal = 0x04AB,
    YouAreTooPowerfulToUsePortal = 0x04AC,
    YouCannotRecallPortal = 0x04AD,
    YouCannotSummonPortal = 0x04AE,
    LockAlreadyUnlocked = 0x04AF,
    YouCannotLockOrUnlockThat = 0x04B0,
    YouCannotLockWhatIsOpen = 0x04B1,
    KeyDoesntFitThisLock = 0x04B2,
    LockUsedTooRecently = 0x04B3,
    YouArentTrainedInLockpicking = 0x04B4,
    AllegianceInfoEmptyName = 0x04B5,
    AllegianceInfoSelf = 0x04B6,
    AllegianceInfoTooRecent = 0x04B7,
    AbuseNoSuchCharacter = 0x04B8,
    AbuseReportedSelf = 0x04B9,
    AbuseComplaintHandled = 0x04BA,
    YouDoNotOwnThatSalvageTool = 0x04BD,
    YouDoNotOwnThatItem = 0x04BE,
    MaterialCannotBeCreated = 0x04C1,
    ItemsAttemptingToSalvageIsInvalid = 0x04C2,
    YouCannotSalvageItemsInTrading = 0x04C3,
    YouMustBeHouseGuestToUsePortal = 0x04C4,
    YourAllegianceRankIsTooLowToUseMagic = 0x04C5,
    YourArcaneLoreIsTooLowToUseMagic = 0x04C7,
    ItemDoesntHaveEnoughMana = 0x04C8,
    YouHaveBeenInPKBattleTooRecently = 0x04CC,
    TradeAiRefuseEmote = 0x04CD,
    YouFailToAlterSkill = 0x04D0,
    FellowshipDeclined = 0x04DB,
    FellowshipTimeout = 0x04DC,
    YouHaveFailedToAlterAttributes = 0x04DD,
    CannotTransferAttributesWhileWieldingItem = 0x04E0,
    YouHaveSucceededTransferringAttributes = 0x04E1,
    HookIsDuplicated = 0x04E2,
    ItemIsWrongTypeForHook = 0x04E3,
    HousingChestIsDuplicated = 0x04E4,
    HookWillBeDeleted = 0x04E5,
    HousingChestWillBeDeleted = 0x04E6,
    CannotSwearAllegianceWhileOwningMansion = 0x04E7,
    YouCantDoThatWhileInTheAir = 0x04EB,
    CannotChangePKStatusWhileRecovering = 0x04EC,
    AdvocatesCannotChangePKStatus = 0x04ED,
    LevelTooLowToChangePKStatusWithObject = 0x04EE,
    LevelTooHighToChangePKStatusWithObject = 0x04EF,
    YouFeelAHarshDissonance = 0x04F0,
    YouArePKAgain = 0x04F1,
    YouAreTemporarilyNoLongerPK = 0x04F2,
    PKLiteMayNotUsePortal = 0x04F3,
    YouArentTrainedInHealing = 0x04FC,
    YouDontOwnThatHealingKit = 0x04FD,
    YouCantHealThat = 0x04FE,
    YouArentReadyToHeal = 0x0500,
    YouCanOnlyHealPlayers = 0x0501,
    LifestoneMagicProtectsYou = 0x0502,
    PortalEnergyProtectsYou = 0x0503,
    YouAreNonPKAgain = 0x0504,
    YoureTooCloseToYourSanctuary = 0x0505,
    CantDoThatTradeInProgress = 0x0506,
    OnlyNonPKsMayEnterPKLite = 0x0507,
    YouAreNowPKLite = 0x0508,
    YouDoNotBelongToAFellowship = 0x050F,
    UsingMaxHooksSilent = 0x0511,
    YouAreNowUsingMaxHooks = 0x0512,
    YouAreNoLongerUsingMaxHooks = 0x0513,
    YouAreNotPermittedToUseThatHook = 0x0516,
    LockedFellowshipCannotRecruitYou = 0x0519,
    ActivationNotAllowedNotOwner = 0x051A,
    TurbineChatIsEnabled = 0x051D,
    YouCannotAddPeopleToHearList = 0x0520,
    YouAreNowDeafTo_Screams = 0x0523,
    YouCanHearAllPlayersOnceAgain = 0x0524,
    YouChickenOut = 0x0526,
    YouCanPossiblySucceed = 0x0527,
    FellowshipIsLocked = 0x0528,
    TradeComplete = 0x0529,
    NotASalvageTool = 0x052A,
    CharacterNotAvailable = 0x052B,
    YouMustWaitToPurchaseHouse = 0x0532,
    YouDoNotHaveAuthorityInAllegiance = 0x0535,
    YouHaveMaxAccountsBanned = 0x0540,
    YouHaveMaxAllegianceOfficers = 0x0545,
    YourAllegianceOfficersHaveBeenCleared = 0x0546,
    YouCannotJoinChannelsWhileGagged = 0x0548,
    YouAreNoLongerAllegianceOfficer = 0x054A,
    YourAllegianceDoesNotHaveHometown = 0x054C,
    HookItemNotUsable_CannotOpen = 0x054E,
    HookItemNotUsable_CanOpen = 0x054F,
    MissileOutOfRange = 0x0550,
    MustPurchaseThroneOfDestinyToUseFunction = 0x0552,
    MustPurchaseThroneOfDestinyToUseItem = 0x0553,
    MustPurchaseThroneOfDestinyToUsePortal = 0x0554,
    MustPurchaseThroneOfDestinyToAccessQuest = 0x0555,
    YouFailedToCompleteAugmentation = 0x0556,
    AugmentationUsedTooManyTimes = 0x0557,
    AugmentationTypeUsedTooManyTimes = 0x0558,
    AugmentationNotEnoughExperience = 0x0559,
    ExitTrainingAcademyToUseCommand = 0x055D,
    OnlyPKsMayUseCommand = 0x055F,
    OnlyPKLiteMayUseCommand = 0x0560,
    MaxFriendsExceeded = 0x0561,
    ThatCharacterNotOnYourFriendsList = 0x0563,
    OnlyHouseOwnerCanUseCommand = 0x0564,
    InvalidAllegianceNameCantBeEmpty = 0x0565,
    InvalidAllegianceNameTooLong = 0x0566,
    InvalidAllegianceNameBadCharacters = 0x0567,
    InvalidAllegianceNameInappropriate = 0x0568,
    InvalidAllegianceNameAlreadyInUse = 0x0569,
    AllegianceNameCleared = 0x056B,
    InvalidAllegianceNameSameName = 0x056C,
    InvalidOfficerLevel = 0x056F,
    AllegianceOfficerTitleIsNotAppropriate = 0x0570,
    AllegianceNameIsTooLong = 0x0571,
    AllegianceOfficerTitlesCleared = 0x0572,
    AllegianceTitleHasIllegalChars = 0x0573,
    YouHaveNotPreApprovedVassals = 0x0579,
    YouHaveClearedPreApprovedVassal = 0x057C,
    CharIsAlreadyGagged = 0x057D,
    CharIsNotCurrentlyGagged = 0x057E,
    YourAllegianceChatPrivilegesRestored = 0x0581,
    TooManyUniqueItems = 0x0584,
    HeritageRequiresSpecificArmor = 0x0585,
    ArmorRequiresSpecificHeritage = 0x0586,
    OlthoiCannotInteractWithThat = 0x0587,
    OlthoiCannotUseLifestones = 0x0588,
    OlthoiVendorLooksInHorror = 0x0589,
    OlthoiCannotJoinFellowship = 0x058B,
    OlthoiCannotJoinAllegiance = 0x058C,
    YouCannotUseThatItem = 0x058D,
    ThisPersonWillNotInteractWithYou = 0x058E,
    OnlyOlthoiMayUsePortal = 0x058F,
    OlthoiMayNotUsePortal = 0x0590,
    YouMayNotUsePortalWithVitae = 0x0591,
    YouMustBeTwoWeeksOldToUsePortal = 0x0592,
    OlthoiCanOnlyRecallToLifestone = 0x0593,
    ContractError = 0x0594,
}

/// The PositionFlags value defines the fields present in the Position structure.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionFlags {
        pub bits: u32,
}

/// Height of the attack.  TODO these need to be verified.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttackHeight {
    High = 0x01,
    Medium = 0x02,
    Low = 0x03,
}

/// Container properties of an item
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ContainerProperties {
    None = 0x00,
    Container = 0x01,
    Foci = 0x02,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttackType {
    Undef = 0x0,
    Punch = 0x0001,
    Thrust = 0x0002,
    Slash = 0x0004,
    Kick = 0x0008,
    OffhandPunch = 0x0010,
    DoubleSlash = 0x0020,
    TripleSlash = 0x0040,
    DoubleThrust = 0x0080,
    TripleThrust = 0x0100,
    OffhandThrust = 0x0200,
    OffhandSlash = 0x0400,
    OffhandDoubleSlash = 0x0800,
    OffhandTripleSlash = 0x1000,
    OffhandDoubleThrust = 0x2000,
    OffhandTripleThrust = 0x4000,
}

/// The objects type information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ItemType {
        pub bits: u32,
}

/// The Skill identifies a specific Character skill.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SkillId {
    Axe = 0x01,
    Bow = 0x02,
    Crossbow = 0x03,
    Dagger = 0x04,
    Mace = 0x05,
    MeleeDefense = 0x06,
    MissileDefense = 0x07,
    Sling = 0x08,
    Spear = 0x09,
    Staff = 0x0A,
    Sword = 0x0B,
    ThrownWeapons = 0x0C,
    UnarmedCombat = 0x0D,
    ArcaneLore = 0x0E,
    MagicDefense = 0x0F,
    ManaConversion = 0x10,
    Spellcraft = 0x11,
    ItemTinkering = 0x12,
    AssessPerson = 0x13,
    Deception = 0x14,
    Healing = 0x15,
    Jump = 0x16,
    Lockpick = 0x17,
    Run = 0x18,
    Awareness = 0x19,
    ArmorRepair = 0x1A,
    AssessCreature = 0x1B,
    WeaponTinkering = 0x1C,
    ArmorTinkering = 0x1D,
    MagicItemTinkering = 0x1E,
    CreatureEnchantment = 0x1F,
    ItemEnchantment = 0x20,
    LifeMagic = 0x21,
    WarMagic = 0x22,
    Leadership = 0x23,
    Loyalty = 0x24,
    Fletching = 0x25,
    Alchemy = 0x26,
    Cooking = 0x27,
    Salvaging = 0x28,
    TwoHandedCombat = 0x29,
    Gearcraft = 0x2A,
    VoidMagic = 0x2B,
    HeavyWeapons = 0x2C,
    LightWeapons = 0x2D,
    FinesseWeapons = 0x2E,
    MissleWeapons = 0x2F,
    DualWield = 0x31,
    Recklessness = 0x32,
    SneakAttack = 0x33,
    DirtyFighting = 0x34,
    Challenge = 0x35,
    Summoning = 0x36,
}

/// The SkillAdvancementClass identifies whether a skill is untrained, trained or specialized.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SkillAdvancementClass {
    Untrained = 0x01,
    Trained = 0x02,
    Specialized = 0x03,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyAttribute2nd {
    Undef = 0x00,
    MaxHealth = 0x01,
    Health = 0x02,
    MaxStamina = 0x03,
    Stamina = 0x04,
    MaxMana = 0x05,
    Mana = 0x06,
}

/// The EmoteType identifies the type of emote action
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EmoteType {
    Invalid_EmoteType = 0x00,
    Invalid_VendorEmoteType = 0x00,
    Act_EmoteType = 0x01,
    AwardXP_EmoteType = 0x02,
    Give_EmoteType = 0x03,
    MoveHome_EmoteType = 0x04,
    Motion_EmoteType = 0x05,
    Move_EmoteType = 0x06,
    PhysScript_EmoteType = 0x07,
    Say_EmoteType = 0x08,
    Sound_EmoteType = 0x09,
    Tell_EmoteType = 0x0A,
    Turn_EmoteType = 0x0B,
    TurnToTarget_EmoteType = 0x0C,
    TextDirect_EmoteType = 0x0D,
    CastSpell_EmoteType = 0x0E,
    Activate_EmoteType = 0x0F,
    WorldBroadcast_EmoteType = 0x10,
    LocalBroadcast_EmoteType = 0x11,
    DirectBroadcast_EmoteType = 0x12,
    CastSpellInstant_EmoteType = 0x13,
    UpdateQuest_EmoteType = 0x14,
    InqQuest_EmoteType = 0x15,
    StampQuest_EmoteType = 0x16,
    StartEvent_EmoteType = 0x17,
    StopEvent_EmoteType = 0x18,
    BLog_EmoteType = 0x19,
    AdminSpam_EmoteType = 0x1A,
    TeachSpell_EmoteType = 0x1B,
    AwardSkillXP_EmoteType = 0x1C,
    AwardSkillPoints_EmoteType = 0x1D,
    InqQuestSolves_EmoteType = 0x1E,
    EraseQuest_EmoteType = 0x1F,
    DecrementQuest_EmoteType = 0x20,
    IncrementQuest_EmoteType = 0x21,
    AddCharacterTitle_EmoteType = 0x22,
    InqBoolStat_EmoteType = 0x23,
    InqIntStat_EmoteType = 0x24,
    InqFloatStat_EmoteType = 0x25,
    InqStringStat_EmoteType = 0x26,
    InqAttributeStat_EmoteType = 0x27,
    InqRawAttributeStat_EmoteType = 0x28,
    InqSecondaryAttributeStat_EmoteType = 0x29,
    InqRawSecondaryAttributeStat_EmoteType = 0x2A,
    InqSkillStat_EmoteType = 0x2B,
    InqRawSkillStat_EmoteType = 0x2C,
    InqSkillTrained_EmoteType = 0x2D,
    InqSkillSpecialized_EmoteType = 0x2E,
    AwardTrainingCredits_EmoteType = 0x2F,
    InflictVitaePenalty_EmoteType = 0x30,
    AwardLevelProportionalXP_EmoteType = 0x31,
    AwardLevelProportionalSkillXP_EmoteType = 0x32,
    InqEvent_EmoteType = 0x33,
    ForceMotion_EmoteType = 0x34,
    SetIntStat_EmoteType = 0x35,
    IncrementIntStat_EmoteType = 0x36,
    DecrementIntStat_EmoteType = 0x37,
    CreateTreasure_EmoteType = 0x38,
    ResetHomePosition_EmoteType = 0x39,
    InqFellowQuest_EmoteType = 0x3A,
    InqFellowNum_EmoteType = 0x3B,
    UpdateFellowQuest_EmoteType = 0x3C,
    StampFellowQuest_EmoteType = 0x3D,
    AwardNoShareXP_EmoteType = 0x3E,
    SetSanctuaryPosition_EmoteType = 0x3F,
    TellFellow_EmoteType = 0x40,
    FellowBroadcast_EmoteType = 0x41,
    LockFellow_EmoteType = 0x42,
    Goto_EmoteType = 0x43,
    PopUp_EmoteType = 0x44,
    SetBoolStat_EmoteType = 0x45,
    SetQuestCompletions_EmoteType = 0x46,
    InqNumCharacterTitles_EmoteType = 0x47,
    Generate_EmoteType = 0x48,
    PetCastSpellOnOwner_EmoteType = 0x49,
    TakeItems_EmoteType = 0x4A,
    InqYesNo_EmoteType = 0x4B,
    InqOwnsItems_EmoteType = 0x4C,
    DeleteSelf_EmoteType = 0x4D,
    KillSelf_EmoteType = 0x4E,
    UpdateMyQuest_EmoteType = 0x4F,
    InqMyQuest_EmoteType = 0x50,
    StampMyQuest_EmoteType = 0x51,
    InqMyQuestSolves_EmoteType = 0x52,
    EraseMyQuest_EmoteType = 0x53,
    DecrementMyQuest_EmoteType = 0x54,
    IncrementMyQuest_EmoteType = 0x55,
    SetMyQuestCompletions_EmoteType = 0x56,
    MoveToPos_EmoteType = 0x57,
    LocalSignal_EmoteType = 0x58,
    InqPackSpace_EmoteType = 0x59,
    RemoveVitaePenalty_EmoteType = 0x5A,
    SetEyeTexture_EmoteType = 0x5B,
    SetEyePalette_EmoteType = 0x5C,
    SetNoseTexture_EmoteType = 0x5D,
    SetNosePalette_EmoteType = 0x5E,
    SetMouthTexture_EmoteType = 0x5F,
    SetMouthPalette_EmoteType = 0x60,
    SetHeadObject_EmoteType = 0x61,
    SetHeadPalette_EmoteType = 0x62,
    TeleportTarget_EmoteType = 0x63,
    TeleportSelf_EmoteType = 0x64,
    StartBarber_EmoteType = 0x65,
    InqQuestBitsOn_EmoteType = 0x66,
    InqQuestBitsOff_EmoteType = 0x67,
    InqMyQuestBitsOn_EmoteType = 0x68,
    InqMyQuestBitsOff_EmoteType = 0x69,
    SetQuestBitsOn_EmoteType = 0x6A,
    SetQuestBitsOff_EmoteType = 0x6B,
    SetMyQuestBitsOn_EmoteType = 0x6C,
    SetMyQuestBitsOff_EmoteType = 0x6D,
    UntrainSkill_EmoteType = 0x6E,
    SetAltRacialSkills_EmoteType = 0x6F,
    SpendLuminance_EmoteType = 0x70,
    AwardLuminance_EmoteType = 0x71,
    InqInt64Stat_EmoteType = 0x72,
    SetInt64Stat_EmoteType = 0x73,
    OpenMe_EmoteType = 0x74,
    CloseMe_EmoteType = 0x75,
    SetFloatStat_EmoteType = 0x76,
    AddContract_EmoteType = 0x77,
    RemoveContract_EmoteType = 0x78,
    InqContractsFull_EmoteType = 0x79,
}

/// The EmoteCategory identifies the category of an emote.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EmoteCategory {
    Invalid_EmoteCategory = 0x00,
    Refuse_EmoteCategory = 0x01,
    Vendor_EmoteCategory = 0x02,
    Death_EmoteCategory = 0x03,
    Portal_EmoteCategory = 0x04,
    HeartBeat_EmoteCategory = 0x05,
    Give_EmoteCategory = 0x06,
    Use_EmoteCategory = 0x07,
    Activation_EmoteCategory = 0x08,
    Generation_EmoteCategory = 0x09,
    PickUp_EmoteCategory = 0x0A,
    Drop_EmoteCategory = 0x0B,
    QuestSuccess_EmoteCategory = 0x0C,
    QuestFailure_EmoteCategory = 0x0D,
    Taunt_EmoteCategory = 0x0E,
    WoundedTaunt_EmoteCategory = 0x0F,
    KillTaunt_EmoteCategory = 0x10,
    NewEnemy_EmoteCategory = 0x11,
    Scream_EmoteCategory = 0x12,
    Homesick_EmoteCategory = 0x13,
    ReceiveCritical_EmoteCategory = 0x14,
    ResistSpell_EmoteCategory = 0x15,
    TestSuccess_EmoteCategory = 0x16,
    TestFailure_EmoteCategory = 0x17,
    HearChat_EmoteCategory = 0x18,
    Wield_EmoteCategory = 0x19,
    UnWield_EmoteCategory = 0x1A,
    EventSuccess_EmoteCategory = 0x1B,
    EventFailure_EmoteCategory = 0x1C,
    TestNoQuality_EmoteCategory = 0x1D,
    QuestNoFellow_EmoteCategory = 0x1E,
    TestNoFellow_EmoteCategory = 0x1F,
    GotoSet_EmoteCategory = 0x20,
    NumFellowsSuccess_EmoteCategory = 0x21,
    NumFellowsFailure_EmoteCategory = 0x22,
    NumCharacterTitlesSuccess_EmoteCategory = 0x23,
    NumCharacterTitlesFailure_EmoteCategory = 0x24,
    ReceiveLocalSignal_EmoteCategory = 0x25,
    ReceiveTalkDirect_EmoteCategory = 0x26,
}

/// The CharacterOptions1 word contains character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterOptions1 {
        pub bits: u32,
}

/// The CharacterOptions2 word contains additional character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CharacterOptions2 {
        pub bits: u32,
}

/// The various options for filtering the spellbook
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpellBookFilterOptions {
        pub bits: u32,
}

/// The EquipMask value describes the equipment slots an item uses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipMask {
        pub bits: u32,
}

/// The type of the friend change event.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FriendsUpdateType {
        pub bits: u32,
}

/// The permission levels that can be given to an allegiance officer
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AllegianceOfficerLevel {
    Speaker = 0x01,
    Seneschal = 0x02,
    Castellan = 0x03,
}

/// Actions related to /allegiance lock
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AllegianceLockAction {
    LockedOff = 0x01,
    LockedOn = 0x02,
    ToggleLocked = 0x03,
    CheckLocked = 0x04,
    DisplayBypass = 0x05,
    ClearBypass = 0x06,
}

/// Actions related to /allegiance house
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AllegianceHouseAction {
    Help = 0x01,
    GuestOpen = 0x02,
    GuestClosed = 0x03,
    StorageOpen = 0x04,
    StorageClosed = 0x05,
}

/// The AttributeId identifies a specific Character attribute.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttributeId {
    Strength = 0x01,
    Endurance = 0x02,
    Quickness = 0x03,
    Coordination = 0x04,
    Focus = 0x05,
    #[serde(rename = "Self")]
    Self_ = 0x06,
}

/// The VitalId identifies a specific Character vital (secondary attribute).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VitalId {
    MaximumHealth = 0x01,
    MaximumStamina = 0x03,
    MaximumMana = 0x05,
}

/// The CurVitalId identifies a specific Character vital (secondary attribute).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CurVitalId {
    CurrentHealth = 0x02,
    CurrentStamina = 0x04,
    CurrentMana = 0x06,
}

/// The combat mode for a character or monster.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatMode {
        pub bits: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Sound {
    Invalid = 0x00,
    Speak1 = 0x01,
    Random = 0x02,
    Attack1 = 0x03,
    Attack2 = 0x04,
    Attack3 = 0x05,
    SpecialAttack1 = 0x06,
    SpecialAttack2 = 0x07,
    SpecialAttack3 = 0x08,
    Damage1 = 0x09,
    Damage2 = 0x0A,
    Damage3 = 0x0B,
    Wound1 = 0x0C,
    Wound2 = 0x0D,
    Wound3 = 0x0E,
    Death1 = 0x0F,
    Death2 = 0x10,
    Death3 = 0x11,
    Grunt1 = 0x12,
    Grunt2 = 0x13,
    Grunt3 = 0x14,
    Oh1 = 0x15,
    Oh2 = 0x16,
    Oh3 = 0x17,
    Heave1 = 0x18,
    Heave2 = 0x19,
    Heave3 = 0x1A,
    Knockdown1 = 0x1B,
    Knockdown2 = 0x1C,
    Knockdown3 = 0x1D,
    Swoosh1 = 0x1E,
    Swoosh2 = 0x1F,
    Swoosh3 = 0x20,
    Thump1 = 0x21,
    Smash1 = 0x22,
    Scratch1 = 0x23,
    Spear = 0x24,
    Sling = 0x25,
    Dagger = 0x26,
    ArrowWhiz1 = 0x27,
    ArrowWhiz2 = 0x28,
    CrossbowPull = 0x29,
    CrossbowRelease = 0x2A,
    BowPull = 0x2B,
    BowRelease = 0x2C,
    ThrownWeaponRelease1 = 0x2D,
    ArrowLand = 0x2E,
    Collision = 0x2F,
    HitFlesh1 = 0x30,
    HitLeather1 = 0x31,
    HitChain1 = 0x32,
    HitPlate1 = 0x33,
    HitMissile1 = 0x34,
    HitMissile2 = 0x35,
    HitMissile3 = 0x36,
    Footstep1 = 0x37,
    Footstep2 = 0x38,
    Walk1 = 0x39,
    Dance1 = 0x3A,
    Dance2 = 0x3B,
    Dance3 = 0x3C,
    Hidden1 = 0x3D,
    Hidden2 = 0x3E,
    Hidden3 = 0x3F,
    Eat1 = 0x40,
    Drink1 = 0x41,
    Open = 0x42,
    Close = 0x43,
    OpenSlam = 0x44,
    CloseSlam = 0x45,
    Ambient1 = 0x46,
    Ambient2 = 0x47,
    Ambient3 = 0x48,
    Ambient4 = 0x49,
    Ambient5 = 0x4A,
    Ambient6 = 0x4B,
    Ambient7 = 0x4C,
    Ambient8 = 0x4D,
    Waterfall = 0x4E,
    LogOut = 0x4F,
    LogIn = 0x50,
    LifestoneOn = 0x51,
    AttribUp = 0x52,
    AttribDown = 0x53,
    SkillUp = 0x54,
    SkillDown = 0x55,
    HealthUp = 0x56,
    HealthDown = 0x57,
    ShieldUp = 0x58,
    ShieldDown = 0x59,
    EnchantUp = 0x5A,
    EnchantDown = 0x5B,
    VisionUp = 0x5C,
    VisionDown = 0x5D,
    Fizzle = 0x5E,
    Launch = 0x5F,
    Explode = 0x60,
    TransUp = 0x61,
    TransDown = 0x62,
    BreatheFlaem = 0x63,
    BreatheAcid = 0x64,
    BreatheFrost = 0x65,
    BreatheLightning = 0x66,
    Create = 0x67,
    Destroy = 0x68,
    Lockpicking = 0x69,
    UI_EnterPortal = 0x6A,
    UI_ExitPortal = 0x6B,
    UI_GeneralQuery = 0x6C,
    UI_GeneralError = 0x6D,
    UI_TransientMessage = 0x6E,
    UI_IconPickUp = 0x6F,
    UI_IconSuccessfulDrop = 0x70,
    UI_IconInvalid_Drop = 0x71,
    UI_ButtonPress = 0x72,
    UI_GrabSlider = 0x73,
    UI_ReleaseSlider = 0x74,
    UI_NewTargetSelected = 0x75,
    UI_Roar = 0x76,
    UI_Bell = 0x77,
    UI_Chant1 = 0x78,
    UI_Chant2 = 0x79,
    UI_DarkWhispers1 = 0x7A,
    UI_DarkWhispers2 = 0x7B,
    UI_DarkLaugh = 0x7C,
    UI_DarkWind = 0x7D,
    UI_DarkSpeech = 0x7E,
    UI_Drums = 0x7F,
    UI_GhostSpeak = 0x80,
    UI_Breathing = 0x81,
    UI_Howl = 0x82,
    UI_LostSouls = 0x83,
    UI_Squeal = 0x84,
    UI_Thunder1 = 0x85,
    UI_Thunder2 = 0x86,
    UI_Thunder3 = 0x87,
    UI_Thunder4 = 0x88,
    UI_Thunder5 = 0x89,
    UI_Thunder6 = 0x8A,
    RaiseTrait = 0x8B,
    WieldObject = 0x8C,
    UnwieldObject = 0x8D,
    ReceiveItem = 0x8E,
    PickUpItem = 0x8F,
    DropItem = 0x90,
    ResistSpell = 0x91,
    PicklockFail = 0x92,
    LockSuccess = 0x93,
    OpenFailDueToLock = 0x94,
    TriggerActivated = 0x95,
    SpellExpire = 0x96,
    ItemManaDepleted = 0x97,
    TriggerActivated1 = 0x98,
    TriggerActivated2 = 0x99,
    TriggerActivated3 = 0x9A,
    TriggerActivated4 = 0x9B,
    TriggerActivated5 = 0x9C,
    TriggerActivated6 = 0x9D,
    TriggerActivated7 = 0x9E,
    TriggerActivated8 = 0x9F,
    TriggerActivated9 = 0xA0,
    TriggerActivated10 = 0xA1,
    TriggerActivated11 = 0xA2,
    TriggerActivated12 = 0xA3,
    TriggerActivated13 = 0xA4,
    TriggerActivated14 = 0xA5,
    TriggerActivated15 = 0xA6,
    TriggerActivated16 = 0xA7,
    TriggerActivated17 = 0xA8,
    TriggerActivated18 = 0xA9,
    TriggerActivated19 = 0xAA,
    TriggerActivated20 = 0xAB,
    TriggerActivated21 = 0xAC,
    TriggerActivated22 = 0xAD,
    TriggerActivated23 = 0xAE,
    TriggerActivated24 = 0xAF,
    TriggerActivated25 = 0xB0,
    TriggerActivated26 = 0xB1,
    TriggerActivated27 = 0xB2,
    TriggerActivated28 = 0xB3,
    TriggerActivated29 = 0xB4,
    TriggerActivated30 = 0xB5,
    TriggerActivated31 = 0xB6,
    TriggerActivated32 = 0xB7,
    TriggerActivated33 = 0xB8,
    TriggerActivated34 = 0xB9,
    TriggerActivated35 = 0xBA,
    TriggerActivated36 = 0xBB,
    TriggerActivated37 = 0xBC,
    TriggerActivated38 = 0xBD,
    TriggerActivated39 = 0xBE,
    TriggerActivated40 = 0xBF,
    TriggerActivated41 = 0xC0,
    TriggerActivated42 = 0xC1,
    TriggerActivated43 = 0xC2,
    TriggerActivated44 = 0xC3,
    TriggerActivated45 = 0xC4,
    TriggerActivated46 = 0xC5,
    TriggerActivated47 = 0xC6,
    TriggerActivated48 = 0xC7,
    TriggerActivated49 = 0xC8,
    TriggerActivated50 = 0xC9,
    HealthDownVoid = 0xCA,
    RegenDownVoid = 0xCB,
    SkillDownVoid = 0xCC,
}

/// The ChatFragmentType categorizes chat window messages to control color and filtering.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChatFragmentType {
    Default = 0x00,
    Speech = 0x02,
    Tell = 0x03,
    OutgoingTell = 0x04,
    System = 0x05,
    Combat = 0x06,
    Magic = 0x07,
    Channels = 0x08,
    OutgoingChannel = 0x09,
    Social = 0x0A,
    OutgoingSocial = 0x0B,
    Emote = 0x0C,
    Advancement = 0x0D,
    Abuse = 0x0E,
    Help = 0x0F,
    Appraisal = 0x10,
    Spellcasting = 0x11,
    Allegiance = 0x12,
    Fellowship = 0x13,
    WorldBroadcast = 0x14,
    CombatEnemy = 0x15,
    CombatSelf = 0x16,
    Recall = 0x17,
    Craft = 0x18,
    Salvaging = 0x19,
    AdminTell = 0x1F,
}

/// Flags related to the use of the item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ObjectDescriptionFlag {
        pub bits: u32,
}

/// The AmmoType value describes the type of ammunition a missile weapon uses.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AmmoType {
        pub bits: u16,
}

/// The useablilty flags of the object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usable {
        pub bits: u32,
}

/// The CoverageMask value describes what parts of the body an item protects.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CoverageMask {
        pub bits: u32,
}

/// The HookType identifies the types of dwelling hooks.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HookType {
        pub bits: u16,
}

/// The MaterialType identifies the material an object is made of.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MaterialType {
    Ceramic = 0x00000001,
    Porcelain = 0x00000002,
    Linen = 0x00000004,
    Satin = 0x00000005,
    Silk = 0x00000006,
    Velvet = 0x00000007,
    Wool = 0x00000008,
    Agate = 0x0000000A,
    Amber = 0x0000000B,
    Amethyst = 0x0000000C,
    Aquamarine = 0x0000000D,
    Azurite = 0x0000000E,
    BlackGarnet = 0x0000000F,
    BlackOpal = 0x00000010,
    Bloodstone = 0x00000011,
    Carnelian = 0x00000012,
    Citrine = 0x00000013,
    Diamond = 0x00000014,
    Emerald = 0x00000015,
    FireOpal = 0x00000016,
    GreenGarnet = 0x00000017,
    GreenJade = 0x00000018,
    Hematite = 0x00000019,
    ImperialTopaz = 0x0000001A,
    Jet = 0x0000001B,
    LapisLazuli = 0x0000001C,
    LavenderJade = 0x0000001D,
    Malachite = 0x0000001E,
    Moonstone = 0x0000001F,
    Onyx = 0x00000020,
    Opal = 0x00000021,
    Peridot = 0x00000022,
    RedGarnet = 0x00000023,
    RedJade = 0x00000024,
    RoseQuartz = 0x00000025,
    Ruby = 0x00000026,
    Sapphire = 0x00000027,
    SmokeyQuartz = 0x00000028,
    Sunstone = 0x00000029,
    TigerEye = 0x0000002A,
    Tourmaline = 0x0000002B,
    Turquoise = 0x0000002C,
    WhiteJade = 0x0000002D,
    WhiteQuartz = 0x0000002E,
    WhiteSapphire = 0x0000002F,
    YellowGarnet = 0x00000030,
    YellowTopaz = 0x00000031,
    Zircon = 0x00000032,
    Ivory = 0x00000033,
    Leather = 0x00000034,
    ArmoredilloHide = 0x00000035,
    GromnieHide = 0x00000036,
    ReedSharkHide = 0x00000037,
    Brass = 0x00000039,
    Bronze = 0x0000003A,
    Copper = 0x0000003B,
    Gold = 0x0000003C,
    Iron = 0x0000003D,
    Pyreal = 0x0000003E,
    Silver = 0x0000003F,
    Steel = 0x00000040,
    Alabaster = 0x00000042,
    Granite = 0x00000043,
    Marble = 0x00000044,
    Obsidian = 0x00000045,
    Sandstone = 0x00000046,
    Serpentine = 0x00000047,
    Ebony = 0x00000049,
    Mahogany = 0x0000004A,
    Oak = 0x0000004B,
    Pine = 0x0000004C,
    Teak = 0x0000004D,
}

/// The ConfirmationType identifies the specific confirmation panel to be displayed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConfirmationType {
    SwearAllegiance = 0x01,
    AlterSkill = 0x02,
    AlterAttribute = 0x03,
    Fellowship = 0x04,
    Craft = 0x05,
    Augmentation = 0x06,
    YesNo = 0x07,
}

/// The EnvrionChangeType identifies the environment option set.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnvrionChangeType {
    Clear = 0x00,
    RedFog = 0x01,
    BlueFog = 0x02,
    WhiteFog = 0x03,
    GreenFog = 0x04,
    BlackFog = 0x05,
    BlackFog2 = 0x06,
    RoarSound = 0x65,
    BellSound = 0x66,
    Chant1Sound = 0x67,
    Chant2Sound = 0x68,
    DarkWhispers1Sound = 0x69,
    DarkWhispers2Sound = 0x6A,
    DarkLaughSound = 0x6B,
    DarkWindSound = 0x6C,
    DarkSpeechSound = 0x6D,
    DrumsSound = 0x6E,
    GhostSpeakSound = 0x6F,
    BreathingSound = 0x70,
    HowlSound = 0x71,
    LostSoulsSound = 0x72,
    SquealSound = 0x75,
    Thunder1Sound = 0x76,
    Thunder2Sound = 0x77,
    Thunder3Sound = 0x78,
    Thunder4Sound = 0x79,
    Thunder5Sound = 0x7A,
    Thunder6Sound = 0x7B,
}

/// The movement type defines the fields for the rest of the message
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MovementType {
    InterpertedMotionState = 0x00,
    MoveToObject = 0x06,
    MoveToPosition = 0x07,
    TurnToObject = 0x08,
    TurnToPosition = 0x09,
}

/// Additional movement options
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MovementOption {
    None = 0x00,
    StickToObject = 0x01,
    StandingLongJump = 0x02,
}

/// Command types
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Command {
    Invalid = 0x00,
    HoldRun = 0x01,
    HoldSidestep = 0x02,
    Ready = 0x03,
    Stop = 0x04,
    WalkForward = 0x05,
    WalkBackwards = 0x06,
    RunForward = 0x07,
    Fallen = 0x08,
    Interpolating = 0x09,
    Hover = 0x0A,
    On = 0x0B,
    Off = 0x0C,
    TurnRight = 0x0D,
    TurnLeft = 0x0E,
    SideStepRight = 0x0F,
    SideStepLeft = 0x10,
    Dead = 0x11,
    Crouch = 0x12,
    Sitting = 0x13,
    Sleeping = 0x14,
    Falling = 0x15,
    Reload = 0x16,
    Unload = 0x17,
    Pickup = 0x18,
    StoreInBackpack = 0x19,
    Eat = 0x1A,
    Drink = 0x1B,
    Reading = 0x1C,
    JumpCharging = 0x1D,
    AimLevel = 0x1E,
    AimHigh15 = 0x1F,
    AimHigh30 = 0x20,
    AimHigh45 = 0x21,
    AimHigh60 = 0x22,
    AimHigh75 = 0x23,
    AimHigh90 = 0x24,
    AimLow15 = 0x25,
    AimLow30 = 0x26,
    AimLow45 = 0x27,
    AimLow60 = 0x28,
    AimLow75 = 0x29,
    AimLow90 = 0x2A,
    MagicBlast = 0x2B,
    MagicSelfHead = 0x2C,
    MagicSelfHeart = 0x2D,
    MagicBonus = 0x2E,
    MagicClap = 0x2F,
    MagicHarm = 0x30,
    MagicHeal = 0x31,
    MagicThrowMissile = 0x32,
    MagicRecoilMissile = 0x33,
    MagicPenalty = 0x34,
    MagicTransfer = 0x35,
    MagicVision = 0x36,
    MagicEnchantItem = 0x37,
    MagicPortal = 0x38,
    MagicPray = 0x39,
    StopTurning = 0x3A,
    Jump = 0x3B,
    HandCombat = 0x3C,
    NonCombat = 0x3D,
    SwordCombat = 0x3E,
    BowCombat = 0x3F,
    SwordShieldCombat = 0x40,
    CrossbowCombat = 0x41,
    UnusedCombat = 0x42,
    SlingCombat = 0x43,
    TwoHandedSwordCombat = 0x44,
    TwoHandedStaffCombat = 0x45,
    DualWieldCombat = 0x46,
    ThrownWeaponCombat = 0x47,
    Graze = 0x48,
    Magi = 0x49,
    Hop = 0x4A,
    Jumpup = 0x4B,
    Cheer = 0x4C,
    ChestBeat = 0x4D,
    TippedLeft = 0x4E,
    TippedRight = 0x4F,
    FallDown = 0x50,
    Twitch1 = 0x51,
    Twitch2 = 0x52,
    Twitch3 = 0x53,
    Twitch4 = 0x54,
    StaggerBackward = 0x55,
    StaggerForward = 0x56,
    Sanctuary = 0x57,
    ThrustMed = 0x58,
    ThrustLow = 0x59,
    ThrustHigh = 0x5A,
    SlashHigh = 0x5B,
    SlashMed = 0x5C,
    SlashLow = 0x5D,
    BackhandHigh = 0x5E,
    BackhandMed = 0x5F,
    BackhandLow = 0x60,
    Shoot = 0x61,
    AttackHigh1 = 0x62,
    AttackMed1 = 0x63,
    AttackLow1 = 0x64,
    AttackHigh2 = 0x65,
    AttackMed2 = 0x66,
    AttackLow2 = 0x67,
    AttackHigh3 = 0x68,
    AttackMed3 = 0x69,
    AttackLow3 = 0x6A,
    HeadThrow = 0x6B,
    FistSlam = 0x6C,
    BreatheFlame_ = 0x6D,
    SpinAttack = 0x6E,
    MagicPowerUp01 = 0x6F,
    MagicPowerUp02 = 0x70,
    MagicPowerUp03 = 0x71,
    MagicPowerUp04 = 0x72,
    MagicPowerUp05 = 0x73,
    MagicPowerUp06 = 0x74,
    MagicPowerUp07 = 0x75,
    MagicPowerUp08 = 0x76,
    MagicPowerUp09 = 0x77,
    MagicPowerUp10 = 0x78,
    ShakeFist = 0x79,
    Beckon = 0x7A,
    BeSeeingYou = 0x7B,
    BlowKiss = 0x7C,
    BowDeep = 0x7D,
    ClapHands = 0x7E,
    Cry = 0x7F,
    Laugh = 0x80,
    MimeEat = 0x81,
    MimeDrink = 0x82,
    Nod = 0x83,
    Point = 0x84,
    ShakeHead = 0x85,
    Shrug = 0x86,
    Wave = 0x87,
    Akimbo = 0x88,
    HeartyLaugh = 0x89,
    Salute = 0x8A,
    ScratchHead = 0x8B,
    SmackHead = 0x8C,
    TapFoot = 0x8D,
    WaveHigh = 0x8E,
    WaveLow = 0x8F,
    YawnStretch = 0x90,
    Cringe = 0x91,
    Kneel = 0x92,
    Plead = 0x93,
    Shiver = 0x94,
    Shoo = 0x95,
    Slouch = 0x96,
    Spit = 0x97,
    Surrender = 0x98,
    Woah = 0x99,
    Winded = 0x9A,
    YMCA = 0x9B,
    EnterGame = 0x9C,
    ExitGame = 0x9D,
    OnCreation = 0x9E,
    OnDestruction = 0x9F,
    EnterPortal = 0xA0,
    ExitPortal = 0xA1,
    Cancel = 0xA2,
    UseSelected = 0xA3,
    AutosortSelected = 0xA4,
    DropSelected = 0xA5,
    GiveSelected = 0xA6,
    SplitSelected = 0xA7,
    ExamineSelected = 0xA8,
    CreateShortcutToSelected = 0xA9,
    PreviousCompassItem = 0xAA,
    NextCompassItem = 0xAB,
    ClosestCompassItem = 0xAC,
    PreviousSelection = 0xAD,
    LastAttacker = 0xAE,
    PreviousFellow = 0xAF,
    NextFellow = 0xB0,
    ToggleCombat = 0xB1,
    HighAttack = 0xB2,
    MediumAttack = 0xB3,
    LowAttack = 0xB4,
    EnterChat = 0xB5,
    ToggleChat = 0xB6,
    SavePosition = 0xB7,
    OptionsPanel = 0xB8,
    ResetView = 0xB9,
    CameraLeftRotate = 0xBA,
    CameraRightRotate = 0xBB,
    CameraRaise = 0xBC,
    CameraLower = 0xBD,
    CameraCloser = 0xBE,
    CameraFarther = 0xBF,
    FloorView = 0xC0,
    MouseLook = 0xC1,
    PreviousItem = 0xC2,
    NextItem = 0xC3,
    ClosestItem = 0xC4,
    ShiftView = 0xC5,
    MapView = 0xC6,
    AutoRun = 0xC7,
    DecreasePowerSetting = 0xC8,
    IncreasePowerSetting = 0xC9,
    Pray = 0xCA,
    Mock = 0xCB,
    Teapot = 0xCC,
    SpecialAttack1 = 0xCD,
    SpecialAttack2 = 0xCE,
    SpecialAttack3 = 0xCF,
    MissileAttack1 = 0xD0,
    MissileAttack2 = 0xD1,
    MissileAttack3 = 0xD2,
    CastSpell = 0xD3,
    Flatulence = 0xD4,
    FirstPersonView = 0xD5,
    AllegiancePanel = 0xD6,
    FellowshipPanel = 0xD7,
    SpellbookPanel = 0xD8,
    SpellComponentsPanel = 0xD9,
    HousePanel = 0xDA,
    AttributesPanel = 0xDB,
    SkillsPanel = 0xDC,
    MapPanel = 0xDD,
    InventoryPanel = 0xDE,
    Demonet = 0xDF,
    UseMagicStaff = 0xE0,
    UseMagicWand = 0xE1,
    Blink = 0xE2,
    Bite = 0xE3,
    TwitchSubstate1 = 0xE4,
    TwitchSubstate2 = 0xE5,
    TwitchSubstate3 = 0xE6,
    CaptureScreenshotToFile = 0xE7,
    BowNoAmmo = 0xE8,
    CrossBowNoAmmo = 0xE9,
    ShakeFistState = 0xEA,
    PrayState = 0xEB,
    BowDeepState = 0xEC,
    ClapHandsState = 0xED,
    CrossArmsState = 0xEE,
    ShiverState = 0xEF,
    PointState = 0xF0,
    WaveState = 0xF1,
    AkimboState = 0xF2,
    SaluteState = 0xF3,
    ScratchHeadState = 0xF4,
    TapFootState = 0xF5,
    LeanState = 0xF6,
    KneelState = 0xF7,
    PleadState = 0xF8,
    ATOYOT = 0xF9,
    SlouchState = 0xFA,
    SurrenderState = 0xFB,
    WoahState = 0xFC,
    WindedState = 0xFD,
    AutoCreateShortcuts = 0xFE,
    AutoRepeatAttacks = 0xFF,
    AutoTarget = 0x100,
    AdvancedCombatInterface = 0x101,
    IgnoreAllegianceRequests = 0x102,
    IgnoreFellowshipRequests = 0x103,
    InvertMouseLook = 0x104,
    LetPlayersGiveYouItems = 0x105,
    AutoTrackCombatTargets = 0x106,
    DisplayTooltips = 0x107,
    AttemptToDeceivePlayers = 0x108,
    RunAsDefaultMovement = 0x109,
    StayInChatModeAfterSend = 0x10A,
    RightClickToMouseLook = 0x10B,
    VividTargetIndicator = 0x10C,
    SelectSelf = 0x10D,
    SkillHealSelf = 0x10E,
    NextMonster = 0x10F,
    PreviousMonster = 0x110,
    ClosestMonster = 0x111,
    NextPlayer = 0x112,
    PreviousPlayer = 0x113,
    ClosestPlayer = 0x114,
    SnowAngelState = 0x115,
    WarmHands = 0x116,
    CurtseyState = 0x117,
    AFKState = 0x118,
    MeditateState = 0x119,
    TradePanel = 0x11A,
    LogOut = 0x11B,
    DoubleSlashLow = 0x11C,
    DoubleSlashMed = 0x11D,
    DoubleSlashHigh = 0x11E,
    TripleSlashLow = 0x11F,
    TripleSlashMed = 0x120,
    TripleSlashHigh = 0x121,
    DoubleThrustLow = 0x122,
    DoubleThrustMed = 0x123,
    DoubleThrustHigh = 0x124,
    TripleThrustLow = 0x125,
    TripleThrustMed = 0x126,
    TripleThrustHigh = 0x127,
    MagicPowerUp01Purple = 0x128,
    MagicPowerUp02Purple = 0x129,
    MagicPowerUp03Purple = 0x12A,
    MagicPowerUp04Purple = 0x12B,
    MagicPowerUp05Purple = 0x12C,
    MagicPowerUp06Purple = 0x12D,
    MagicPowerUp07Purple = 0x12E,
    MagicPowerUp08Purple = 0x12F,
    MagicPowerUp09Purple = 0x130,
    MagicPowerUp10Purple = 0x131,
    Helper = 0x132,
    Pickup5 = 0x133,
    Pickup10 = 0x134,
    Pickup15 = 0x135,
    Pickup20 = 0x136,
    HouseRecall = 0x137,
    AtlatlCombat = 0x138,
    ThrownShieldCombat = 0x139,
    SitState = 0x13A,
    SitCrossleggedState = 0x13B,
    SitBackState = 0x13C,
    PointLeftState = 0x13D,
    PointRightState = 0x13E,
    TalktotheHandState = 0x13F,
    PointDownState = 0x140,
    DrudgeDanceState = 0x141,
    PossumState = 0x142,
    ReadState = 0x143,
    ThinkerState = 0x144,
    HaveASeatState = 0x145,
    AtEaseState = 0x146,
    NudgeLeft = 0x147,
    NudgeRight = 0x148,
    PointLeft = 0x149,
    PointRight = 0x14A,
    PointDown = 0x14B,
    Knock = 0x14C,
    ScanHorizon = 0x14D,
    DrudgeDance = 0x14E,
    HaveASeat = 0x14F,
    LifestoneRecall = 0x150,
    CharacterOptionsPanel = 0x151,
    SoundAndGraphicsPanel = 0x152,
    HelpfulSpellsPanel = 0x153,
    HarmfulSpellsPanel = 0x154,
    CharacterInformationPanel = 0x155,
    LinkStatusPanel = 0x156,
    VitaePanel = 0x157,
    ShareFellowshipXP = 0x158,
    ShareFellowshipLoot = 0x159,
    AcceptCorpseLooting = 0x15A,
    IgnoreTradeRequests = 0x15B,
    DisableWeather = 0x15C,
    DisableHouseEffect = 0x15D,
    SideBySideVitals = 0x15E,
    ShowRadarCoordinates = 0x15F,
    ShowSpellDurations = 0x160,
    MuteOnLosingFocus = 0x161,
    Fishing = 0x162,
    MarketplaceRecall = 0x163,
    EnterPKLite = 0x164,
    AllegianceChat = 0x165,
    AutomaticallyAcceptFellowshipRequests = 0x166,
    Reply = 0x167,
    MonarchReply = 0x168,
    PatronReply = 0x169,
    ToggleCraftingChanceOfSuccessDialog = 0x16A,
    UseClosestUnopenedCorpse = 0x16B,
    UseNextUnopenedCorpse = 0x16C,
    IssueSlashCommand = 0x16D,
    AllegianceHometownRecall = 0x16E,
    PKArenaRecall = 0x16F,
    OffhandSlashHigh = 0x170,
    OffhandSlashMed = 0x171,
    OffhandSlashLow = 0x172,
    OffhandThrustHigh = 0x173,
    OffhandThrustMed = 0x174,
    OffhandThrustLow = 0x175,
    OffhandDoubleSlashLow = 0x176,
    OffhandDoubleSlashMed = 0x177,
    OffhandDoubleSlashHigh = 0x178,
    OffhandTripleSlashLow = 0x179,
    OffhandTripleSlashMed = 0x17A,
    OffhandTripleSlashHigh = 0x17B,
    OffhandDoubleThrustLow = 0x17C,
    OffhandDoubleThrustMed = 0x17D,
    OffhandDoubleThrustHigh = 0x17E,
    OffhandTripleThrustLow = 0x17F,
    OffhandTripleThrustMed = 0x180,
    OffhandTripleThrustHigh = 0x181,
    OffhandKick = 0x182,
    AttackHigh4 = 0x183,
    AttackMed4 = 0x184,
    AttackLow4 = 0x185,
    AttackHigh5 = 0x186,
    AttackMed5 = 0x187,
    AttackLow5 = 0x188,
    AttackHigh6 = 0x189,
    AttackMed6 = 0x18A,
    AttackLow6 = 0x18B,
    PunchFastHigh = 0x18C,
    PunchFastMed = 0x18D,
    PunchFastLow = 0x18E,
    PunchSlowHigh = 0x18F,
    PunchSlowMed = 0x190,
    PunchSlowLow = 0x191,
    OffhandPunchFastHigh = 0x192,
    OffhandPunchFastMed = 0x193,
    OffhandPunchFastLow = 0x194,
    OffhandPunchSlowHigh = 0x195,
    OffhandPunchSlowMed = 0x196,
    OffhandPunchSlowLow = 0x197,
}

/// The stance for a character or monster.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum StanceMode {
    HandCombat = 0x3C,
    NonCombat = 0x3D,
    SwordCombat = 0x3E,
    BowCombat = 0x3F,
    SwordShieldCombat = 0x40,
    CrossbowCombat = 0x41,
    UnusedCombat = 0x42,
    SlingCombat = 0x43,
    TwoHandedSwordCombat = 0x44,
    TwoHandedStaffCombat = 0x45,
    DualWieldCombat = 0x46,
    ThrownWeaponCombat = 0x47,
    BowNoAmmo = 0xE8,
    CrossBowNoAmmo = 0xE9,
    AtlatlCombat = 0x138,
    ThrownShieldCombat = 0x139,
}

/// The movement (forward, side, turn) for a character or monster.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MovementCommand {
    HoldRun = 0x01,
    HoldSidestep = 0x02,
    WalkForward = 0x05,
    WalkBackwards = 0x06,
    RunForward = 0x07,
    TurnRight = 0x0D,
    TurnLeft = 0x0E,
    SideStepRight = 0x0F,
    SideStepLeft = 0x10,
}

/// House flags
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HouseBitfield {
    Undef = 0x0,
    Active = 0x1,
    RequiresMonarch = 0x2,
}

/// The type response to a chargen request
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CharGenResponseType {
    OK = 0x0001,
    NameInUse = 0x0003,
    NameBanned = 0x0004,
    Corrupt = 0x0005,
    Corrupt_0x0006 = 0x0006,
    AdminPrivilegeDenied = 0x0007,
}

/// The CharacterErrorType identifies the type of character error that has occured.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CharacterErrorType {
    Logon = 0x01,
    AccountLogin = 0x03,
    ServerCrash = 0x04,
    Logoff = 0x05,
    Delete = 0x06,
    ServerCrash2 = 0x08,
    AccountInvalid = 0x09,
    AccountDoesntExist = 0x0A,
    EnterGameGeneric = 0x0B,
    EnterGameStressAccount = 0x0C,
    EnterGameCharacterInWorld = 0x0D,
    EnterGamePlayerAccountMissing = 0x0E,
    EnterGameCharacterNotOwned = 0x0F,
    EnterGameCharacterInWorldServer = 0x10,
    EnterGameOldCharacter = 0x11,
    EnterGameCorruptCharacter = 0x12,
    EnterGameStartServerDown = 0x13,
    EnterGameCouldntPlaceCharacter = 0x14,
    LogonServerFull = 0x15,
    EnterGameCharacterLocked = 0x17,
    SubscriptionExpired = 0x18,
}

/// The state flags for an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicsState {
        pub bits: u32,
}

/// The TurbineChatType identifies the type of Turbine Chat message.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TurbineChatType {
    ServerToClientMessage = 0x01,
    ClientToServerMessage = 0x03,
    AckClientToServerMessage = 0x05,
}

/// The DatFileType identifies the dat file to be used.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DatFileType {
    client_portal = 0x01,
    client_cell_1 = 0x02,
    client_local_English = 0x03,
}

/// The CompressionType identifies the type of data compression used.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CompressionType {
    None = 0x00,
    ZLib = 0x01,
}

/// The AttributeMask selects which creature attributes highlighting is applied to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeMask {
        pub bits: u16,
}

/// The DamageType identifies the type of damage.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DamageType {
        pub bits: u32,
}

/// The HookAppraisalFlags identifies various properties for an item hooked.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HookAppraisalFlags {
        pub bits: u32,
}

/// The ArmorHighlightMask selects which armor attributes highlighting is applied to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmorHighlightMask {
        pub bits: u16,
}

/// The ResistHighlightMask selects which wand attributes highlighting is applied to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResistHighlightMask {
        pub bits: u16,
}

/// The WeaponHighlightMask selects which weapon attributes highlighting is applied to.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeaponHighlightMask {
        pub bits: u16,
}

/// Additional attack information
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttackConditionsMask {
        pub bits: u32,
}

/// The DamageLocation indicates where damage was done.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum DamageLocation {
    Head = 0x00,
    Chest = 0x01,
    Abdomen = 0x02,
    UpperArm = 0x03,
    LowerArm = 0x04,
    Hand = 0x05,
    UpperLeg = 0x06,
    LowerLeg = 0x07,
    Foot = 0x08,
}

/// The LogTextType indicates the kind of text going to the chat area.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogTextType {
    Default = 0x00,
    Speech = 0x02,
    Tell = 0x03,
    SpeechDirectSend = 0x04,
    System = 0x05,
    Combat = 0x06,
    Magic = 0x07,
    Channel = 0x08,
    ChannelSend = 0x09,
    Social = 0x0A,
    SocialSend = 0x0B,
    Emote = 0x0C,
    Advancement = 0x0D,
    Abuse = 0x0E,
    Help = 0x0F,
    Appraisal = 0x10,
    Spellcasting = 0x11,
    Allegiance = 0x12,
    Fellowship = 0x13,
    WorldBroadcast = 0x14,
    CombatEnemy = 0x15,
    CombatSelf = 0x16,
    Recall = 0x17,
    Craft = 0x18,
    Salvaging = 0x19,
    AdminTell = 0x1F,
}

/// The EndTradeReason identifies the reason trading was ended.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EndTradeReason {
    Normal = 0x00,
    EnteredCombat = 0x02,
    Cancelled = 0x51,
}

/// The TradeSide identifies the side of the trade window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TradeSide {
    #[serde(rename = "Self")]
    Self_ = 0x01,
    Partner = 0x02,
}

/// The HouseType identifies the type of house.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HouseType {
    Cottage = 0x01,
    Villa = 0x02,
    Mansion = 0x03,
    Apartment = 0x04,
}

/// Identifies the chess move attempt result.  Negative/0 values are failures.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChessMoveResult {
    FailureNotYourTurn = -3,
    FailureInvalidDirection = -100,
    FailureInvalidDistance = -101,
    FailureMovingEmptySquare = -102,
    FailureMovingOpponentPiece = -103,
    FailureMovedPieceOffBoard = -104,
    FailureAttackingOwnPiece = -105,
    FailureCannotMoveIntoCheck = -106,
    FailurePathBlocked = -107,
    FailureCastleOutOfCheck = -108,
    FailureCastleThroughCheck = -109,
    FailureCastlePieceMoved = -110,
    FailureInvalidMove = 0,
    Success = 0x1,
    OpponentInCheck = 0x400,
    CheckMatedOpponent = 0x800,
}

/// Type of fellow update
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FellowUpdateType {
    FullUpdate = 0x01,
    UpdateStats = 0x02,
    UpdateVitals = 0x03,
}

/// Stage a contract is in.  Values 4+ appear to provide contract specific update messages
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ContractStage {
    New = 0x01,
    InProgress = 0x02,
    DoneOrPendingRepeat = 0x03,
}

/// Movement hold key
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HoldKey {
    Invalid = 0x00,
    None = 0x01,
    Run = 0x02,
}

/// Radar behavior
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RadarBehavior {
    Undefined = 0x00,
    ShowNever = 0x01,
    ShowMovement = 0x02,
    ShowAttacking = 0x03,
    ShowAlways = 0x04,
}

/// Gender of a player
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Invalid = 0x00,
    Male = 0x01,
    Female = 0x02,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FactionBits {
        pub bits: u32,
}

/// Creature type
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CreatureType {
    Olthoi = 1,
    Banderling = 2,
    Drudge = 3,
    Mosswart = 4,
    Lugian = 5,
    Tumerok = 6,
    Mite = 7,
    Tusker = 8,
    PhyntosWasp = 9,
    Rat = 10,
    Auroch = 11,
    Cow = 12,
    Golem = 13,
    Undead = 14,
    Gromnie = 15,
    Reedshark = 16,
    Armoredillo = 17,
    Fae = 18,
    Virindi = 19,
    Wisp = 20,
    Knathtead = 21,
    Shadow = 22,
    Mattekar = 23,
    Mumiyah = 24,
    Rabbit = 25,
    Sclavus = 26,
    ShallowsShark = 27,
    Monouga = 28,
    Zefir = 29,
    Skeleton = 30,
    Human = 31,
    Shreth = 32,
    Chittick = 33,
    Moarsman = 34,
    OlthoiLarvae = 35,
    Slithis = 36,
    Deru = 37,
    FireElemental = 38,
    Snowman = 39,
    Unknown = 40,
    Bunny = 41,
    LightningElemental = 42,
    Rockslide = 43,
    Grievver = 44,
    Niffis = 45,
    Ursuin = 46,
    Crystal = 47,
    HollowMinion = 48,
    Scarecrow = 49,
    Idol = 50,
    Empyrean = 51,
    Hopeslayer = 52,
    Doll = 53,
    Marionette = 54,
    Carenzi = 55,
    Siraluun = 56,
    AunTumerok = 57,
    HeaTumerok = 58,
    Simulacrum = 59,
    AcidElemental = 60,
    FrostElemental = 61,
    Elemental = 62,
    Statue = 63,
    Wall = 64,
    AlteredHuman = 65,
    Device = 66,
    Harbinger = 67,
    DarkSarcophagus = 68,
    Chicken = 69,
    GotrokLugian = 70,
    Margul = 71,
    BleachedRabbit = 72,
    NastyRabbit = 73,
    GrimacingRabbit = 74,
    Burun = 75,
    Target = 76,
    Ghost = 77,
    Fiun = 78,
    Eater = 79,
    Penguin = 80,
    Ruschk = 81,
    Thrungus = 82,
    ViamontianKnight = 83,
    Remoran = 84,
    Swarm = 85,
    Moar = 86,
    EnchantedArms = 87,
    Sleech = 88,
    Mukkir = 89,
    Merwart = 90,
    Food = 91,
    ParadoxOlthoi = 92,
    Harvest = 93,
    Energy = 94,
    Apparition = 95,
    Aerbax = 96,
    Touched = 97,
    BlightedMoarsman = 98,
    GearKnight = 99,
    Gurog = 100,
    Anekshay = 101,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CombatStyle {
        pub bits: u32,
}

/// Indicates what data is present in the ACQualities data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ACQualitiesFlags {
        pub bits: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GeneratorDestruct {
    Undef = 0,
    Nothing = 1,
    Destroy = 2,
    Kill = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GeneratorTimeType {
    Undef = 0,
    RealTime = 1,
    Defined = 2,
    Event = 3,
    Night = 4,
    Day = 5,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum GeneratorType {
    Undef = 0,
    Relative = 1,
    Absolute = 2,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ImbuedEffectType {
    Undef = 0,
    CriticalStrike = 0x0001,
    CripplingBlow = 0x0002,
    ArmorRending = 0x0004,
    SlashRending = 0x0008,
    PierceRending = 0x0010,
    BludgeonRending = 0x0020,
    AcidRending = 0x0040,
    ColdRending = 0x0080,
    ElectricRending = 0x0100,
    FireRending = 0x0200,
    MeleeDefense = 0x0400,
    MissileDefense = 0x0800,
    MagicDefense = 0x1000,
    Spellbook = 0x2000,
    NetherRending = 0x4000,
    IgnoreSomeMagicProjectileDamage = 0x20000000,
    AlwaysCritical = 0x40000000,
    IgnoreAllArmor = 0x80000000,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ItemXpStyle {
    Undef = 0,
    Fixed = 1,
    ScalesWithLevel = 2,
    FixedPlusBase = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SubscriptionStatus {
    No_Subscription = 0,
    AsheronsCall_Subscription = 1,
    DarkMajesty_Subscription = 2,
    ThroneOfDestiny_Subscription = 3,
    ThroneOfDestiny_Preordered = 4,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WeaponType {
    Undef = 0,
    Unarmed = 1,
    Sword = 2,
    Axe = 3,
    Mace = 4,
    Spear = 5,
    Dagger = 6,
    Staff = 7,
    Bow = 8,
    Crossbow = 9,
    Thrown = 10,
    TwoHanded = 11,
    Magic = 12,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ActivationResponse {
    Undef = 0,
    Use = 0x2,
    Animate = 0x4,
    Talk = 0x10,
    Emote = 0x800,
    CastSpell = 0x1000,
    Generate = 0x10000,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AetheriaBitfield {
    None = 0,
    Blue = 0x1,
    Yellow = 0x2,
    Red = 0x4,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HookGroupType {
    Undef = 0x0,
    NoisemakingItems = 0x1,
    TestItems = 0x2,
    PortalItems = 0x4,
    WritableItems = 0x8,
    SpellCastingItems = 0x10,
    SpellTeachingItems = 0x20,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ArmorType {
    None = 0,
    Cloth = 1,
    Leather = 2,
    StuddedLeather = 4,
    Scalemail = 8,
    Chainmail = 16,
    Metal = 32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum AttunedStatus {
    Normal = 0,
    Attuned = 1,
    Sticky = 2,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BondedStatus {
    Destroy = -2,
    Slippery = -1,
    Normal = 0,
    Bonded = 1,
    Sticky = 2,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HouseStatus {
    Disabled = -1,
    InActive = 0,
    Active = 1,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UiEffects {
        pub bits: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PortalBitmask {
    Undef = 0x00,
    NotPassable = 0x00,
    Unrestricted = 0x01,
    NoPk = 0x02,
    NoPKLite = 0x04,
    NoNPK = 0x08,
    NoSummon = 0x10,
    NoRecall = 0x20,
    OnlyOlthoiPCs = 0x40,
    NoOlthoiPCs = 0x80,
    NoVitae = 0x100,
    NoNewAccounts = 0x200,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WieldRequirement {
    Undef = 0,
    Skill = 1,
    RawSkill = 2,
    Attrib = 3,
    RawAttrib = 4,
    SecondaryAttrib = 5,
    RawSecondaryAttrib = 6,
    Level = 7,
    Training = 8,
    IntStat = 9,
    BoolStat = 10,
    CreatureType = 11,
    HeritageType = 12,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PaletteTemplate {
    Undef = 0,
    AquaBlue = 1,
    Blue = 2,
    BluePurple = 3,
    Brown = 4,
    DarkBlue = 5,
    DeepBrown = 6,
    DeepGreen = 7,
    Green = 8,
    Grey = 9,
    LightBlue = 10,
    Maroon = 11,
    Navy = 12,
    Purple = 13,
    Red = 14,
    RedPurple = 15,
    Rose = 16,
    Yellow = 17,
    YellowBrown = 18,
    Copper = 19,
    Silver = 20,
    Gold = 21,
    Aqua = 22,
    DarkAquaMetal = 23,
    DarkBlueMetal = 24,
    DarkCopperMetal = 25,
    DarkGoldMetal = 26,
    DarkGreenMetal = 27,
    DarkPurpleMetal = 28,
    DarkRedMetal = 29,
    DarkSilverMetal = 30,
    LightAquaMetal = 31,
    LightBlueMetal = 32,
    LightCopperMetal = 33,
    LightGoldMetal = 34,
    LightGreenMetal = 35,
    LightPurpleMetal = 36,
    LightRedMetal = 37,
    LightSilverMetal = 38,
    Black = 39,
    Bronze = 40,
    SandyYellow = 41,
    DarkBrown = 42,
    LightBrown = 43,
    TanRed = 44,
    PaleGreen = 45,
    Tan = 46,
    PastyYellow = 47,
    SnowyWhite = 48,
    RuddyYellow = 49,
    RuddierYellow = 50,
    MidGrey = 51,
    DarkGrey = 52,
    BlueDullSilver = 53,
    YellowPaleSilver = 54,
    BrownBlueDark = 55,
    BrownBlueMed = 56,
    GreenSilver = 57,
    BrownGreen = 58,
    YellowGreen = 59,
    PalePurple = 60,
    White = 61,
    RedBrown = 62,
    GreenBrown = 63,
    OrangeBrown = 64,
    PaleGreenBrown = 65,
    PaleOrange = 66,
    GreenSlime = 67,
    BlueSlime = 68,
    YellowSlime = 69,
    PurpleSlime = 70,
    DullRed = 71,
    GreyWhite = 72,
    MediumGrey = 73,
    DullGreen = 74,
    OliveGreen = 75,
    Orange = 76,
    BlueGreen = 77,
    Olive = 78,
    Lead = 79,
    Iron = 80,
    LiteGreen = 81,
    PinkPurple = 82,
    Amber = 83,
    DyeDarkGreen = 84,
    DyeDarkRed = 85,
    DyeDarkYellow = 86,
    DyeBotched = 87,
    DyeWinterBlue = 88,
    DyeWinterGreen = 89,
    DyeWinterSilver = 90,
    DyeSpringBlue = 91,
    DyeSpringPurple = 92,
    DyeSpringBlack = 93,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SummoningMastery {
    Undef = 0,
    Primalist = 1,
    Necromancer = 2,
    Naturalist = 3,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ContractId {
    Undef = 0,
    Contract_1_The_Shadows_of_Bitter_Winter = 1,
    Contract_2_Test_Quest_Stamping = 2,
    Contract_3_Test_Contract_3 = 3,
    Contract_4_Test_Contract_4 = 4,
    Contract_5_Reign_of_Terror = 5,
    Contract_6_Glenden_Wood_Invasion_Low = 6,
    Contract_7_Glenden_Wood_Invasion_Mid = 7,
    Contract_8_Glenden_Wood_Invasion_High = 8,
    Contract_9_Frozen_Fury = 9,
    Contract_10_Defense_of_Zaikhal_Copper = 10,
    Contract_11_Defense_of_Zaikhal_Silver = 11,
    Contract_12_Defense_of_Zaikhal_Gold = 12,
    Contract_13_Defense_of_Zaikhal_Platinum = 13,
    Contract_14_The_Caliginous_Bethel = 14,
    Contract_15_The_Legend_of_the_Tusker_Paw = 15,
    Contract_16_Oswalds_Lair = 16,
    Contract_17_The_Decrepit_Tower = 17,
    Contract_18_Banderling_Haunt = 18,
    Contract_19_Reconnaissance = 19,
    Contract_20_Assault_Low = 20,
    Contract_21_Assault_Mid = 21,
    Contract_22_Assault_High = 22,
    Contract_23_Assault_Expert = 23,
    Contract_24_Infiltration = 24,
    Contract_25_Of_Trust_and_Betrayal = 25,
    Contract_26_Ishaqs_Lost_Key = 26,
    Contract_27_The_Shadows_of_Bitter_Winter = 27,
    Contract_28_Suzuhara_Baijins_Delivery = 28,
    Contract_29_Haleatan_Beach_Camps = 29,
    Contract_30_Ricardos_Blood_Gem = 30,
    Contract_31_Sawato_Extortion = 31,
    Contract_32_First_Contact = 32,
    Contract_33_Crafting_Forges_Low = 33,
    Contract_34_Crafting_Forges_Mid = 34,
    Contract_35_Crafting_Forges_High = 35,
    Contract_36_Northern_Shroud_Cabal = 36,
    Contract_37_Southern_Shroud_Cabal = 37,
    Contract_38_Faces_of_the_Mukkir_Low = 38,
    Contract_39_Faces_of_the_Mukkir_Mid = 39,
    Contract_40_Faces_of_the_Mukkir_High = 40,
    Contract_41_Faces_of_the_Mukkir_Expert = 41,
    Contract_42_Fiun_Healing_Machine = 42,
    Contract_43_Hamuds_Demise = 43,
    Contract_44_Raising_Graels_Island = 44,
    Contract_45_Enricos_Betrayal = 45,
    Contract_46_Lost_Pet = 46,
    Contract_47_His_Masters_Voice = 47,
    Contract_48_Tentacles_of_Tthuun = 48,
    Contract_49_Reign_of_Terror = 49,
    Contract_50_The_Crystal_Staff_of_the_Anekshay = 50,
    Contract_51_The_Crystal_Sword_of_the_Anekshay = 51,
    Contract_52_The_Crystal_Amulet_of_the_Anekshay = 52,
    Contract_53_The_Crystal_Idol_of_the_Anekshay = 53,
    Contract_54_Armoredillo_Hunting__Lost_City_of_Neftet = 54,
    Contract_55_Golem_Hunting__Lost_City_of_Neftet = 55,
    Contract_56_Mu_miyah_Hunting__Lost_City_of_Neftet = 56,
    Contract_57_Reedshark_Hunting__Lost_City_of_Neftet = 57,
    Contract_58_Anekshay_Bracer_Collecting__Lost_City_of_Neftet = 58,
    Contract_59_Stone_Tablet_Collecting__Lost_City_of_Neftet = 59,
    Contract_60_Prickly_Pear_Collecting__Lost_City_of_Neftet = 60,
    Contract_61_Contracts__Brokers = 61,
    Contract_62_Aug__Sir_Bellas = 62,
    Contract_63_Aug__Society = 63,
    Contract_64_Aug__Diemos = 64,
    Contract_65_Aug__Luminance = 65,
    Contract_66_Colosseum = 66,
    Contract_67_Aerbaxs_Defeat = 67,
    Contract_68_Summoning_Tthuun = 68,
    Contract_69_Empyrean_Rescue = 69,
    Contract_70_Uncovering_the_Renegades = 70,
    Contract_71_Tumerok_Salted_Meat = 71,
    Contract_72_Deewains_Dark_Cavern = 72,
    Contract_73_Sealing_Away_the_Book_of_Eibhil = 73,
    Contract_74_Soc__Dark_Isle_Delivery = 74,
    Contract_75_Soc__Vaeshok = 75,
    Contract_76_Soc__Shambling_Archivist = 76,
    Contract_77_Soc__Undead_Jaw_Collection = 77,
    Contract_78_Soc__Wight_Blade_Sorcerers = 78,
    Contract_79_Soc__Black_Coral_Collection = 79,
    Contract_80_Soc__Dark_Isle_Scouting = 80,
    Contract_81_Soc__Bandit_Mana_Hunter_Boss = 81,
    Contract_82_Soc__Mana_Infused_Jungle_Flowers = 82,
    Contract_83_Soc__Jungle_Lilies = 83,
    Contract_84_Soc__Moar_Glands = 84,
    Contract_85_Soc__Blessed_Moarsmen = 85,
    Contract_86_Soc__Phyntos_Hive_Splinters = 86,
    Contract_87_Soc__Phyntos_Honey = 87,
    Contract_88_Soc__Phyntos_Queen = 88,
    Contract_89_Soc__Phyntos_Larvae = 89,
    Contract_90_Soc__Killer_Phyntos_Wasps = 90,
    Contract_91_Soc__Coral_Towers = 91,
    Contract_92_Soc__Magshuth_Moarsmen = 92,
    Contract_93_Soc__Moarsman_High_Priest = 93,
    Contract_94_Soc__Artifact_Collection = 94,
    Contract_95_Soc__Moguth_Moarsmen = 95,
    Contract_96_Soc__Shoguth_Moarsmen = 96,
    Contract_97_Soc__Spawning_Pools = 97,
    Contract_98_Soc__Graveyard_Delivery = 98,
    Contract_99_Soc__Stone_Tracings = 99,
    Contract_100_Soc__Falatacot_Reports = 100,
    Contract_101_Soc__Dark_Isle_Delivery = 101,
    Contract_102_Soc__Vaeshok = 102,
    Contract_103_Soc__Shambling_Archivist = 103,
    Contract_104_Soc__Undead_Jaw_Collection = 104,
    Contract_105_Soc__Wight_Blade_Sorcerers = 105,
    Contract_106_Soc__Black_Coral_Collection = 106,
    Contract_107_Soc__Dark_Isle_Scouting = 107,
    Contract_108_Soc__Bandit_Mana_Hunter_Boss = 108,
    Contract_109_Soc__Mana_Infused_Jungle_Flowers = 109,
    Contract_110_Soc__Jungle_Lilies = 110,
    Contract_111_Soc__Moar_Glands = 111,
    Contract_112_Soc__Blessed_Moarsmen = 112,
    Contract_113_Soc__Phyntos_Hive_Splinters = 113,
    Contract_114_Soc__Phyntos_Honey = 114,
    Contract_115_Soc__Phyntos_Queen = 115,
    Contract_116_Soc__Phyntos_Larvae = 116,
    Contract_117_Soc__Killer_Phyntos_Wasps = 117,
    Contract_118_Soc__Coral_Towers = 118,
    Contract_119_Soc__Magshuth_Moarsmen = 119,
    Contract_120_Soc__Moarsman_High_Priest = 120,
    Contract_121_Soc__Artifact_Collection = 121,
    Contract_122_Soc__Moguth_Moarsmen = 122,
    Contract_123_Soc__Shoguth_Moarsmen = 123,
    Contract_124_Soc__Spawning_Pools = 124,
    Contract_125_Soc__Graveyard_Delivery = 125,
    Contract_126_Soc__Stone_Tracings = 126,
    Contract_127_Soc__Falatacot_Reports = 127,
    Contract_128_Soc__Dark_Isle_Delivery = 128,
    Contract_129_Soc__Vaeshok = 129,
    Contract_130_Soc__Shambling_Archivist = 130,
    Contract_131_Soc__Undead_Jaw_Collection = 131,
    Contract_132_Soc__Wight_Blade_Sorcerers = 132,
    Contract_133_Soc__Black_Coral_Collection = 133,
    Contract_134_Soc__Dark_Isle_Scouting = 134,
    Contract_135_Soc__Bandit_Mana_Hunter_Boss = 135,
    Contract_136_Soc__Mana_Infused_Jungle_Flowers = 136,
    Contract_137_Soc__Jungle_Lilies = 137,
    Contract_138_Soc__Moar_Glands = 138,
    Contract_139_Soc__Blessed_Moarsmen = 139,
    Contract_140_Soc__Phyntos_Hive_Splinters = 140,
    Contract_141_Soc__Phyntos_Honey = 141,
    Contract_142_Soc__Phyntos_Queen = 142,
    Contract_143_Soc__Phyntos_Larvae = 143,
    Contract_144_Soc__Killer_Phyntos_Wasps = 144,
    Contract_145_Soc__Coral_Towers = 145,
    Contract_146_Soc__Magshuth_Moarsmen = 146,
    Contract_147_Soc__Moarsman_High_Priest = 147,
    Contract_148_Soc__Artifact_Collection = 148,
    Contract_149_Soc__Moguth_Moarsmen = 149,
    Contract_150_Soc__Shoguth_Moarsmen = 150,
    Contract_151_Soc__Spawning_Pools = 151,
    Contract_152_Soc__Graveyard_Delivery = 152,
    Contract_153_Soc__Stone_Tracings = 153,
    Contract_154_Soc__Falatacot_Reports = 154,
    Contract_155_Soc__Palm_Fort = 155,
    Contract_156_Soc__Supply_Saboteur = 156,
    Contract_157_Soc__Forgotten_Tunnels_of_Nyrleha = 157,
    Contract_158_Soc__Palm_Fort = 158,
    Contract_159_Soc__Supply_Saboteur = 159,
    Contract_160_Soc__Forgotten_Tunnels_of_Nyrleha = 160,
    Contract_161_Soc__Palm_Fort = 161,
    Contract_162_Soc__Supply_Saboteur = 162,
    Contract_163_Soc__Forgotten_Tunnels_of_Nyrleha = 163,
    Contract_164_Kill__Tenebrous_Rifts = 164,
    Contract_165_Kill__Umbral_Rifts = 165,
    Contract_166_Harlunes_Diplomacy = 166,
    Contract_167_Saving_Asheron = 167,
    Contract_168_Menhir_Research = 168,
    Contract_169_Gear_Knight_Excavation = 169,
    Contract_170_Nexus_Crawl = 170,
    Contract_171_Jester_Released = 171,
    Contract_172_Vision_Quest = 172,
    Contract_173_Aerbaxs_Prodigal_Monouga = 173,
    Contract_174_QotM__Weekly_1 = 174,
    Contract_175_QotM__Weekly_2 = 175,
    Contract_176_QotM__Weekly_3 = 176,
    Contract_177_Deaths_Allure = 177,
    Contract_178_Yanshi_Tunnels = 178,
    Contract_179_Kill__Gurog_Minions = 179,
    Contract_180_Kill__Gurog_Soldiers = 180,
    Contract_181_Kill__Gurog_Henchmen = 181,
    Contract_182_Aerbaxs_Prodigal_Tusker = 182,
    Contract_183_Find_the_Barkeeper = 183,
    Contract_184_Find_the_Barkeeper = 184,
    Contract_185_Find_the_Barkeeper = 185,
    Contract_186_Find_the_Barkeeper = 186,
    Contract_187_Find_the_Pathwarden = 187,
    Contract_188_Find_the_Pathwarden = 188,
    Contract_189_Find_the_Pathwarden = 189,
    Contract_190_Find_the_Pathwarden = 190,
    Contract_191_Drudge_Hideout = 191,
    Contract_192_Holtburg_Redoubt = 192,
    Contract_193_The_Beacon = 193,
    Contract_194_The_Missing_Necklace = 194,
    Contract_195_Braid_Mansion_Ruin = 195,
    Contract_196_Nen_Ais_Pet_Drudge = 196,
    Contract_197_Sea_Temple_Catacombs = 197,
    Contract_198_Under_Cove_Crypt = 198,
    Contract_199_Facility_Hub = 199,
    Contract_200_Jailbreak__Ardent_Leader = 200,
    Contract_201_Jailbreak__Blessed_Leader = 201,
    Contract_202_Jailbreak__Verdant_Leader = 202,
    Contract_203_Jailbreak__General_Population = 203,
    Contract_204_Gurog_Creation = 204,
    Contract_205_Wardley_and_the_Wights = 205,
    Contract_206_Aetherium_Ore_Collection = 206,
    Contract_207_Aetherium_Power_Core_Collection = 207,
    Contract_208_Aetherium_Raid_High = 208,
    Contract_209_Soc__Mana_Siphon_Destruction = 209,
    Contract_210_Kill__Gear_Knight_Knights = 210,
    Contract_211_Kill__Gear_Knight_Commander = 211,
    Contract_212_Nalicanas_Test = 212,
    Contract_213_Bloodstone_Investigation = 213,
    Contract_214_Chasing_Oswald = 214,
    Contract_215_Hunting_Aun_Ralirea = 215,
    Contract_216_Aerbaxs_Prodigal_Monouga = 216,
    Contract_217_Aerbaxs_Prodigal_Drudge = 217,
    Contract_218_Aerbaxs_Prodigal_Human = 218,
    Contract_219_Kidnapped_Handmaiden = 219,
    Contract_220_Sepulcher_of_Nightmares = 220,
    Contract_221_Mhoire_Castle = 221,
    Contract_222_Bobos_Medicine = 222,
    Contract_223_Mhoire_Oubliette = 223,
    Contract_224_Geraines_Study = 224,
    Contract_225_Geraines_Hosts = 225,
    Contract_226_Splitting_Grael_High = 226,
    Contract_227_Splitting_Grael_Mid = 227,
    Contract_228_Splitting_Grael_Low = 228,
    Contract_229_Clutch_of_Kings__Reeshan = 229,
    Contract_230_Clutch_of_Kings__Kiree = 230,
    Contract_231_Clutch_of_Kings__Broodu = 231,
    Contract_232_Clutch_of_Kings__Keerik = 232,
    Contract_233_Clutch_of_Kings__Rehir = 233,
    Contract_234_Clutch_of_Kings__Browerk = 234,
    Contract_235_Clutch_of_Kings__All = 235,
    Contract_236_Kill__Spectral_Archers = 236,
    Contract_237_Kill__Spectral_Minions = 237,
    Contract_238_Kill__Spectral_Nanjou_Shou_jen = 238,
    Contract_239_Kill__Spectral_Mages = 239,
    Contract_240_Kill__Spectral_Bushi = 240,
    Contract_241_Kill__Spectral_Samurai = 241,
    Contract_242_Kill__Spectral_Blades_and_Claws = 242,
    Contract_243_Kill__Spectral_Samurai_Golems = 243,
    Contract_244_Hoshino_Fortress = 244,
    Contract_245_Stipend__General = 245,
    Contract_246_Stipend__Celestial_Hand = 246,
    Contract_247_Stipend__Radiant_Blood = 247,
    Contract_248_Stipend__Eldrytch_Web = 248,
    Contract_249_Jester_Focuses = 249,
    Contract_250_Unleash_the_Gearknights = 250,
    Contract_251_Virindi_Rescue = 251,
    Contract_252_Ninja_Academy = 252,
    Contract_253_Tanada_Slaughter = 253,
    Contract_254_Tanada_Intercept = 254,
    Contract_255_Crystalline_Adventurer = 255,
    Contract_256_Crystalline_Markers = 256,
    Contract_257_Crystalline_Killer = 257,
    Contract_258_Crystalline_Bound_Wisp = 258,
    Contract_259_Nanjou_Stockade = 259,
    Contract_260_Mage_Academy = 260,
    Contract_261_Apostate_Finale = 261,
    Contract_262_Lunnums_Return = 262,
    Contract_263_Lunnums_Pyre = 263,
    Contract_264_Lunnums_Disappearance = 264,
    Contract_265_Lost_Lore = 265,
    Contract_266_Sisters_of_Light = 266,
    Contract_267_First_Sister = 267,
    Contract_268_Second_Sister = 268,
    Contract_269_Third_Sister = 269,
    Contract_270_Ritual_Investigation = 270,
    Contract_271_Ritual_Disruption = 271,
    Contract_272_Defeat_Hoshino_Kei = 272,
    Contract_273_Protecting_Picketed_Pets = 273,
    Contract_274_Buried_Alive = 274,
    Contract_275_Graverobber = 275,
    Contract_276_Escape = 276,
    Contract_277_Deconstruction = 277,
    Contract_278_Uziz_Abductions = 278,
    Contract_279_Golem_Hunters__Mud_Golem_Sludge_Lord = 279,
    Contract_280_Golem_Hunters__Copper_Golem_Kingpin = 280,
    Contract_281_Golem_Hunters__Glacial_Golem_Margrave = 281,
    Contract_282_Golem_Hunters__Magma_Golem_Exarch = 282,
    Contract_283_Golem_Hunters__Coral_Golem_Viceroy = 283,
    Contract_284_Golem_Hunters__Platinum_Golem_Mountain_King = 284,
    Contract_285_Olthoi_Hive_Queen = 285,
    Contract_286_Soc__Mana_Siphon_Destruction = 286,
    Contract_287_Soc__Mana_Siphon_Destruction = 287,
    Contract_288_Soc__Destroy_The_Phalanx = 288,
    Contract_289_Soc__Destroy_The_Phalanx = 289,
    Contract_290_Soc__Destroy_The_Phalanx = 290,
    Contract_291_Soc__Collect_Gear_Knight_Parts = 291,
    Contract_292_Soc__Collect_Gear_Knight_Parts = 292,
    Contract_293_Soc__Collect_Gear_Knight_Parts = 293,
    Contract_294_Kill__Gear_Knight_Squires = 294,
    Contract_295_Behind_The_Mask = 295,
    Contract_296_Frozen_Fortress_Laboratory = 296,
    Contract_297_Frozen_Fortress_Testing_Grounds = 297,
    Contract_298_Olthoi_Hive_Warrior_Pincer = 298,
    Contract_299_Olthoi_Hive_Eviscerator_Pincer = 299,
    Contract_300_Snow_Tusker_Leader_Tusk = 300,
    Contract_301_Journey_To_Madness = 301,
    Contract_302_Visitors = 302,
    Contract_303_Kill__Rynthid_Minions = 303,
    Contract_304_Kill__Empowered_Wisps = 304,
    Contract_305_Kill__Rynthid_Rare_Boss = 305,
    Contract_306_Kill__Rynthid_Slayers = 306,
    Contract_307_Kill__Rynthid_Ragers = 307,
    Contract_308_Kill__Rynthid_Sorcerers = 308,
    Contract_309_Kill__Rynthid_Rifts = 309,
    Contract_310_Legendary_Quests = 310,
    Contract_311_Rynthid_Genesis = 311,
    Contract_312_Changing_Gears = 312,
    Contract_313_Fear_Factory = 313,
    Contract_314_Spirited_Halls = 314,
    Contract_315_End_of_Days = 315,
    Contract_316_Lugian_Assault = 316,
    Contract_317_Rynthid_Training = 317,
    Contract_318_Kill__Tou_Tou_Shadow_Flyers = 318,
    Contract_319_Kill__Tou_Tou_Grievver_Shredders = 319,
    Contract_320_Kill__Tou_Tou_Devourer_Marguls = 320,
    Contract_321_Kill__Tou_Tou_Shadows = 321,
    Contract_322_Kill__Tou_Tou_Void_Lords = 322,
}

/// The PropertyInt64 identifies a specific Character or Object int64 property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyInt64 {
    TotalExperience = 0x0001,
    AvailableExperience = 0x0002,
    AugmentationCost = 0x0003,
    ItemTotalXp = 0x0004,
    ItemBaseXp = 0x0005,
    AvailableLuminance = 0x0006,
    MaximumLuminance = 0x0007,
    InteractionReqs = 0x0008,
}

/// The PropertyBool identifies a specific Character or Object boolean property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyBool {
    Undef = 0,
    Stuck = 1,
    Open = 2,
    Locked = 3,
    RotProof = 4,
    AllegianceUpdateRequest = 5,
    AiUsesMana = 6,
    AiUseHumanMagicAnimations = 7,
    AllowGive = 8,
    CurrentlyAttacking = 9,
    AttackerAi = 10,
    IgnoreCollisions = 11,
    ReportCollisions = 12,
    Ethereal = 13,
    GravityStatus = 14,
    LightsStatus = 15,
    ScriptedCollision = 16,
    Inelastic = 17,
    Visibility = 18,
    Attackable = 19,
    SafeSpellComponents = 20,
    AdvocateState = 21,
    Inscribable = 22,
    DestroyOnSell = 23,
    UiHidden = 24,
    IgnoreHouseBarriers = 25,
    HiddenAdmin = 26,
    PkWounder = 27,
    PkKiller = 28,
    NoCorpse = 29,
    UnderLifestoneProtection = 30,
    ItemManaUpdatePending = 31,
    GeneratorStatus = 32,
    ResetMessagePending = 33,
    DefaultOpen = 34,
    DefaultLocked = 35,
    DefaultOn = 36,
    OpenForBusiness = 37,
    IsFrozen = 38,
    DealMagicalItems = 39,
    LogoffImDead = 40,
    ReportCollisionsAsEnvironment = 41,
    AllowEdgeSlide = 42,
    AdvocateQuest = 43,
    IsAdmin = 44,
    IsArch = 45,
    IsSentinel = 46,
    IsAdvocate = 47,
    CurrentlyPoweringUp = 48,
    GeneratorEnteredWorld = 49,
    NeverFailCasting = 50,
    VendorService = 51,
    AiImmobile = 52,
    DamagedByCollisions = 53,
    IsDynamic = 54,
    IsHot = 55,
    IsAffecting = 56,
    AffectsAis = 57,
    SpellQueueActive = 58,
    GeneratorDisabled = 59,
    IsAcceptingTells = 60,
    LoggingChannel = 61,
    OpensAnyLock = 62,
    UnlimitedUse = 63,
    GeneratedTreasureItem = 64,
    IgnoreMagicResist = 65,
    IgnoreMagicArmor = 66,
    AiAllowTrade = 67,
    SpellComponentsRequired = 68,
    IsSellable = 69,
    IgnoreShieldsBySkill = 70,
    NoDraw = 71,
    ActivationUntargeted = 72,
    HouseHasGottenPriorityBootPos = 73,
    GeneratorAutomaticDestruction = 74,
    HouseHooksVisible = 75,
    HouseRequiresMonarch = 76,
    HouseHooksEnabled = 77,
    HouseNotifiedHudOfHookCount = 78,
    AiAcceptEverything = 79,
    IgnorePortalRestrictions = 80,
    RequiresBackpackSlot = 81,
    DontTurnOrMoveWhenGiving = 82,
    NpcLooksLikeObject = 83,
    IgnoreCloIcons = 84,
    AppraisalHasAllowedWielder = 85,
    ChestRegenOnClose = 86,
    LogoffInMinigame = 87,
    PortalShowDestination = 88,
    PortalIgnoresPkAttackTimer = 89,
    NpcInteractsSilently = 90,
    Retained = 91,
    IgnoreAuthor = 92,
    Limbo = 93,
    AppraisalHasAllowedActivator = 94,
    ExistedBeforeAllegianceXpChanges = 95,
    IsDeaf = 96,
    IsPsr = 97,
    Invincible = 98,
    Ivoryable = 99,
    Dyable = 100,
    CanGenerateRare = 101,
    CorpseGeneratedRare = 102,
    NonProjectileMagicImmune = 103,
    ActdReceivedItems = 104,
    Unknown105 = 105,
    FirstEnterWorldDone = 106,
    RecallsDisabled = 107,
    RareUsesTimer = 108,
    ActdPreorderReceivedItems = 109,
    Afk = 110,
    IsGagged = 111,
    ProcSpellSelfTargeted = 112,
    IsAllegianceGagged = 113,
    EquipmentSetTriggerPiece = 114,
    Uninscribe = 115,
    WieldOnUse = 116,
    ChestClearedWhenClosed = 117,
    NeverAttack = 118,
    SuppressGenerateEffect = 119,
    TreasureCorpse = 120,
    EquipmentSetAddLevel = 121,
    BarberActive = 122,
    TopLayerPriority = 123,
    NoHeldItemShown = 124,
    LoginAtLifestone = 125,
    OlthoiPk = 126,
    Account15Days = 127,
    HadNoVitae = 128,
    NoOlthoiTalk = 129,
    AutowieldLeft = 130,
}

/// The DataPropertyId identifies a specific Character or Object data property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyDataId {
    Setup = 1,
    MotionTable = 2,
    SoundTable = 3,
    CombatTable = 4,
    QualityFilter = 5,
    PaletteBase = 6,
    ClothingBase = 7,
    Icon = 8,
    EyesTexture = 9,
    NoseTexture = 10,
    MouthTexture = 11,
    DefaultEyesTexture = 12,
    DefaultNoseTexture = 13,
    DefaultMouthTexture = 14,
    HairPalette = 15,
    EyesPalette = 16,
    SkinPalette = 17,
    HeadObject = 18,
    ActivationAnimation = 19,
    InitMotion = 20,
    ActivationSound = 21,
    PhysicsEffectTable = 22,
    UseSound = 23,
    UseTargetAnimation = 24,
    UseTargetSuccessAnimation = 25,
    UseTargetFailureAnimation = 26,
    UseUserAnimation = 27,
    Spell = 28,
    SpellComponent = 29,
    PhysicsScript = 30,
    LinkedPortalOne = 31,
    WieldedTreasureType = 32,
    InventoryTreasureType = 33,
    ShopTreasureType = 34,
    DeathTreasureType = 35,
    MutateFilter = 36,
    ItemSkillLimit = 37,
    UseCreateItem = 38,
    DeathSpell = 39,
    VendorsClassId = 40,
    ItemSpecializedOnly = 41,
    HouseId = 42,
    AccountHouseId = 43,
    RestrictionEffect = 44,
    CreationMutationFilter = 45,
    TsysMutationFilter = 46,
    LastPortal = 47,
    LinkedPortalTwo = 48,
    OriginalPortal = 49,
    IconOverlay = 50,
    IconOverlaySecondary = 51,
    IconUnderlay = 52,
    AugmentationMutationFilter = 53,
    AugmentationEffect = 54,
    ProcSpell = 55,
    AugmentationCreateItem = 56,
    AlternateCurrency = 57,
    BlueSurgeSpell = 58,
    YellowSurgeSpell = 59,
    RedSurgeSpell = 60,
    OlthoiDeathTreasureType = 61,
}

/// The PropertyInt identifies a specific Character or Object int property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyInt {
    ItemType = 1,
    CreatureType = 2,
    PaletteTemplate = 3,
    ClothingPriority = 4,
    EncumbranceVal = 5,
    ItemsCapacity = 6,
    ContainersCapacity = 7,
    Mass = 8,
    ValidLocations = 9,
    CurrentWieldedLocation = 10,
    MaxStackSize = 11,
    StackSize = 12,
    StackUnitEncumbrance = 13,
    StackUnitMass = 14,
    StackUnitValue = 15,
    ItemUseable = 16,
    RareId = 17,
    UiEffects = 18,
    Value = 19,
    CoinValue = 20,
    TotalExperience = 21,
    AvailableCharacter = 22,
    TotalSkillCredits = 23,
    AvailableSkillCredits = 24,
    Level = 25,
    AccountRequirements = 26,
    ArmorType = 27,
    ArmorLevel = 28,
    AllegianceCpPool = 29,
    AllegianceRank = 30,
    ChannelsAllowed = 31,
    ChannelsActive = 32,
    Bonded = 33,
    MonarchsRank = 34,
    AllegianceFollowers = 35,
    ResistMagic = 36,
    ResistItemAppraisal = 37,
    ResistLockpick = 38,
    DeprecatedResistRepair = 39,
    CombatMode = 40,
    CurrentAttackHeight = 41,
    CombatCollisions = 42,
    NumDeaths = 43,
    Damage = 44,
    DamageType = 45,
    DefaultCombatStyle = 46,
    AttackType = 47,
    WeaponSkill = 48,
    WeaponTime = 49,
    AmmoType = 50,
    CombatUse = 51,
    ParentLocation = 52,
    PlacementPosition = 53,
    WeaponEncumbrance = 54,
    WeaponMass = 55,
    ShieldValue = 56,
    ShieldEncumbrance = 57,
    MissileInventoryLocation = 58,
    FullDamageType = 59,
    WeaponRange = 60,
    AttackersSkill = 61,
    DefendersSkill = 62,
    AttackersSkillValue = 63,
    AttackersClass = 64,
    Placement = 65,
    CheckpointStatus = 66,
    Tolerance = 67,
    TargetingTactic = 68,
    CombatTactic = 69,
    HomesickTargetingTactic = 70,
    NumFollowFailures = 71,
    FriendType = 72,
    FoeType = 73,
    MerchandiseItemTypes = 74,
    MerchandiseMinValue = 75,
    MerchandiseMaxValue = 76,
    NumItemsSold = 77,
    NumItemsBought = 78,
    MoneyIncome = 79,
    MoneyOutflow = 80,
    MaxGeneratedObjects = 81,
    InitGeneratedObjects = 82,
    ActivationResponse = 83,
    OriginalValue = 84,
    NumMoveFailures = 85,
    MinLevel = 86,
    MaxLevel = 87,
    LockpickMod = 88,
    BoosterEnum = 89,
    BoostValue = 90,
    MaxStructure = 91,
    Structure = 92,
    PhysicsState = 93,
    TargetType = 94,
    RadarBlipColor = 95,
    EncumbranceCapacity = 96,
    LoginTimestamp = 97,
    CreationTimestamp = 98,
    PkLevelModifier = 99,
    GeneratorType = 100,
    AiAllowedCombatStyle = 101,
    LogoffTimestamp = 102,
    GeneratorDestructionType = 103,
    ActivationCreateClass = 104,
    ItemWorkmanship = 105,
    ItemSpellcraft = 106,
    ItemCurMana = 107,
    ItemMaxMana = 108,
    ItemDifficulty = 109,
    ItemAllegianceRankLimit = 110,
    PortalBitmask = 111,
    AdvocateLevel = 112,
    Gender = 113,
    Attuned = 114,
    ItemSkillLevelLimit = 115,
    GateLogic = 116,
    ItemManaCost = 117,
    Logoff = 118,
    Active = 119,
    AttackHeight = 120,
    NumAttackFailures = 121,
    AiCpThreshold = 122,
    AiAdvancementStrategy = 123,
    Version = 124,
    Age = 125,
    VendorHappyMean = 126,
    VendorHappyVariance = 127,
    CloakStatus = 128,
    VitaeCpPool = 129,
    NumServicesSold = 130,
    MaterialType = 131,
    NumAllegianceBreaks = 132,
    ShowableOnRadar = 133,
    PlayerKillerStatus = 134,
    VendorHappyMaxItems = 135,
    ScorePageNum = 136,
    ScoreConfigNum = 137,
    ScoreNumScores = 138,
    DeathLevel = 139,
    AiOptions = 140,
    OpenToEveryone = 141,
    GeneratorTimeType = 142,
    GeneratorStartTime = 143,
    GeneratorEndTime = 144,
    GeneratorEndDestructionType = 145,
    XpOverride = 146,
    NumCrashAndTurns = 147,
    ComponentWarningThreshold = 148,
    HouseStatus = 149,
    HookPlacement = 150,
    HookType = 151,
    HookItemType = 152,
    AiPpThreshold = 153,
    GeneratorVersion = 154,
    HouseType = 155,
    PickupEmoteOffset = 156,
    WeenieIteration = 157,
    WieldRequirements = 158,
    WieldSkillType = 159,
    WieldDifficulty = 160,
    HouseMaxHooksUsable = 161,
    HouseCurrentHooksUsable = 162,
    AllegianceMinLevel = 163,
    AllegianceMaxLevel = 164,
    HouseRelinkHookCount = 165,
    SlayerCreatureType = 166,
    ConfirmationInProgress = 167,
    ConfirmationTypeInProgress = 168,
    TsysMutationData = 169,
    NumItemsInMaterial = 170,
    NumTimesTinkered = 171,
    AppraisalLongDescDecoration = 172,
    AppraisalLockpickSuccessPercent = 173,
    AppraisalPages = 174,
    AppraisalMaxPages = 175,
    AppraisalItemSkill = 176,
    GemCount = 177,
    GemType = 178,
    ImbuedEffect = 179,
    AttackersRawSkillValue = 180,
    ChessRank = 181,
    ChessTotalGames = 182,
    ChessGamesWon = 183,
    ChessGamesLost = 184,
    TypeOfAlteration = 185,
    SkillToBeAltered = 186,
    SkillAlterationCount = 187,
    HeritageGroup = 188,
    TransferFromAttribute = 189,
    TransferToAttribute = 190,
    AttributeTransferCount = 191,
    FakeFishingSkill = 192,
    NumKeys = 193,
    DeathTimestamp = 194,
    PkTimestamp = 195,
    VictimTimestamp = 196,
    HookGroup = 197,
    AllegianceSwearTimestamp = 198,
    HousePurchaseTimestamp = 199,
    RedirectableEquippedArmorCount = 200,
    MeleeDefenseImbuedEffectTypeCache = 201,
    MissileDefenseImbuedEffectTypeCache = 202,
    MagicDefenseImbuedEffectTypeCache = 203,
    ElementalDamageBonus = 204,
    ImbueAttempts = 205,
    ImbueSuccesses = 206,
    CreatureKills = 207,
    PlayerKillsPk = 208,
    PlayerKillsPkl = 209,
    RaresTierOne = 210,
    RaresTierTwo = 211,
    RaresTierThree = 212,
    RaresTierFour = 213,
    RaresTierFive = 214,
    AugmentationStat = 215,
    AugmentationFamilyStat = 216,
    AugmentationInnateFamily = 217,
    AugmentationInnateStrength = 218,
    AugmentationInnateEndurance = 219,
    AugmentationInnateCoordination = 220,
    AugmentationInnateQuickness = 221,
    AugmentationInnateFocus = 222,
    AugmentationInnateSelf = 223,
    AugmentationSpecializeSalvaging = 224,
    AugmentationSpecializeItemTinkering = 225,
    AugmentationSpecializeArmorTinkering = 226,
    AugmentationSpecializeMagicItemTinkering = 227,
    AugmentationSpecializeWeaponTinkering = 228,
    AugmentationExtraPackSlot = 229,
    AugmentationIncreasedCarryingCapacity = 230,
    AugmentationLessDeathItemLoss = 231,
    AugmentationSpellsRemainPastDeath = 232,
    AugmentationCriticalDefense = 233,
    AugmentationBonusXp = 234,
    AugmentationBonusSalvage = 235,
    AugmentationBonusImbueChance = 236,
    AugmentationFasterRegen = 237,
    AugmentationIncreasedSpellDuration = 238,
    AugmentationResistanceFamily = 239,
    AugmentationResistanceSlash = 240,
    AugmentationResistancePierce = 241,
    AugmentationResistanceBlunt = 242,
    AugmentationResistanceAcid = 243,
    AugmentationResistanceFire = 244,
    AugmentationResistanceFrost = 245,
    AugmentationResistanceLightning = 246,
    RaresTierOneLogin = 247,
    RaresTierTwoLogin = 248,
    RaresTierThreeLogin = 249,
    RaresTierFourLogin = 250,
    RaresTierFiveLogin = 251,
    RaresLoginTimestamp = 252,
    RaresTierSix = 253,
    RaresTierSeven = 254,
    RaresTierSixLogin = 255,
    RaresTierSevenLogin = 256,
    ItemAttributeLimit = 257,
    ItemAttributeLevelLimit = 258,
    ItemAttribute2ndLimit = 259,
    ItemAttribute2ndLevelLimit = 260,
    CharacterTitleId = 261,
    NumCharacterTitles = 262,
    ResistanceModifierType = 263,
    FreeTinkersBitfield = 264,
    EquipmentSetId = 265,
    PetClass = 266,
    Lifespan = 267,
    RemainingLifespan = 268,
    UseCreateQuantity = 269,
    WieldRequirements2 = 270,
    WieldSkillType2 = 271,
    WieldDifficulty2 = 272,
    WieldRequirements3 = 273,
    WieldSkillType3 = 274,
    WieldDifficulty3 = 275,
    WieldRequirements4 = 276,
    WieldSkillType4 = 277,
    WieldDifficulty4 = 278,
    Unique = 279,
    SharedCooldown = 280,
    Faction1Bits = 281,
    Faction2Bits = 282,
    Faction3Bits = 283,
    Hatred1Bits = 284,
    Hatred2Bits = 285,
    Hatred3Bits = 286,
    SocietyRankCelhan = 287,
    SocietyRankEldweb = 288,
    SocietyRankRadblo = 289,
    HearLocalSignals = 290,
    HearLocalSignalsRadius = 291,
    Cleaving = 292,
    AugmentationSpecializeGearcraft = 293,
    AugmentationInfusedCreatureMagic = 294,
    AugmentationInfusedItemMagic = 295,
    AugmentationInfusedLifeMagic = 296,
    AugmentationInfusedWarMagic = 297,
    AugmentationCriticalExpertise = 298,
    AugmentationCriticalPower = 299,
    AugmentationSkilledMelee = 300,
    AugmentationSkilledMissile = 301,
    AugmentationSkilledMagic = 302,
    ImbuedEffect2 = 303,
    ImbuedEffect3 = 304,
    ImbuedEffect4 = 305,
    ImbuedEffect5 = 306,
    DamageRating = 307,
    DamageResistRating = 308,
    AugmentationDamageBonus = 309,
    AugmentationDamageReduction = 310,
    ImbueStackingBits = 311,
    HealOverTime = 312,
    CritRating = 313,
    CritDamageRating = 314,
    CritResistRating = 315,
    CritDamageResistRating = 316,
    HealingResistRating = 317,
    DamageOverTime = 318,
    ItemMaxLevel = 319,
    ItemXpStyle = 320,
    EquipmentSetExtra = 321,
    AetheriaBitfield = 322,
    HealingBoostRating = 323,
    HeritageSpecificArmor = 324,
    AlternateRacialSkills = 325,
    AugmentationJackOfAllTrades = 326,
    AugmentationResistanceNether = 327,
    AugmentationInfusedVoidMagic = 328,
    WeaknessRating = 329,
    NetherOverTime = 330,
    NetherResistRating = 331,
    LuminanceAward = 332,
    LumAugDamageRating = 333,
    LumAugDamageReductionRating = 334,
    LumAugCritDamageRating = 335,
    LumAugCritReductionRating = 336,
    LumAugSurgeEffectRating = 337,
    LumAugSurgeChanceRating = 338,
    LumAugItemManaUsage = 339,
    LumAugItemManaGain = 340,
    LumAugVitality = 341,
    LumAugHealingRating = 342,
    LumAugSkilledCraft = 343,
    LumAugSkilledSpec = 344,
    LumAugNoDestroyCraft = 345,
    RestrictInteraction = 346,
    OlthoiLootTimestamp = 347,
    OlthoiLootStep = 348,
    UseCreatesContractId = 349,
    DotResistRating = 350,
    LifeResistRating = 351,
    CloakWeaveProc = 352,
    WeaponType = 353,
    MeleeMastery = 354,
    RangedMastery = 355,
    SneakAttackRating = 356,
    RecklessnessRating = 357,
    DeceptionRating = 358,
    CombatPetRange = 359,
    WeaponAuraDamage = 360,
    WeaponAuraSpeed = 361,
    SummoningMastery = 362,
    HeartbeatLifespan = 363,
    UseLevelRequirement = 364,
    LumAugAllSkills = 365,
    UseRequiresSkill = 366,
    UseRequiresSkillLevel = 367,
    UseRequiresSkillSpec = 368,
    UseRequiresLevel = 369,
    GearDamage = 370,
    GearDamageResist = 371,
    GearCrit = 372,
    GearCritResist = 373,
    GearCritDamage = 374,
    GearCritDamageResist = 375,
    GearHealingBoost = 376,
    GearNetherResist = 377,
    GearLifeResist = 378,
    GearMaxHealth = 379,
    Unknown380 = 380,
    PKDamageRating = 381,
    PKDamageResistRating = 382,
    GearPKDamageRating = 383,
    GearPKDamageResistRating = 384,
    Unknown385 = 385,
    Overpower = 386,
    OverpowerResist = 387,
    GearOverpower = 388,
    GearOverpowerResist = 389,
    Enlightenment = 390,
}

/// The PropertyInstanceId identifies a specific Character or Object instance property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyInstanceId {
    Owner = 1,
    Container = 2,
    Wielder = 3,
    Freezer = 4,
    Viewer = 5,
    Generator = 6,
    Scribe = 7,
    CurrentCombatTarget = 8,
    CurrentEnemy = 9,
    ProjectileLauncher = 10,
    CurrentAttacker = 11,
    CurrentDamager = 12,
    CurrentFollowTarget = 13,
    CurrentAppraisalTarget = 14,
    CurrentFellowshipAppraisalTarget = 15,
    ActivationTarget = 16,
    Creator = 17,
    Victim = 18,
    Killer = 19,
    Vendor = 20,
    Customer = 21,
    Bonded = 22,
    Wounder = 23,
    Allegiance = 24,
    Patron = 25,
    Monarch = 26,
    CombatTarget = 27,
    HealthQueryTarget = 28,
    LastUnlocker = 29,
    CrashAndTurnTarget = 30,
    AllowedActivator = 31,
    HouseOwner = 32,
    House = 33,
    Slumlord = 34,
    ManaQueryTarget = 35,
    CurrentGame = 36,
    RequestedAppraisalTarget = 37,
    AllowedWielder = 38,
    AssignedTarget = 39,
    LimboSource = 40,
    Snooper = 41,
    TeleportedCharacter = 42,
    Pet = 43,
    PetOwner = 44,
    PetDevice = 45,
}

/// The PropertyPosition identifies a specific Character or Object position property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyPosition {
    Location = 1,
    Destination = 2,
    Instantiation = 3,
    Sanctuary = 4,
    Home = 5,
    ActivationMove = 6,
    Target = 7,
    LinkedPortalOne = 8,
    LastPortal = 9,
    PortalStorm = 10,
    CrashAndTurn = 11,
    PortalSummonLoc = 12,
    HouseBoot = 13,
    LastOutsideDeath = 14,
    LinkedLifestone = 15,
    LinkedPortalTwo = 16,
    Save1 = 17,
    Save2 = 18,
    Save3 = 19,
    Save4 = 20,
    Save5 = 21,
    Save6 = 22,
    Save7 = 23,
    Save8 = 24,
    Save9 = 25,
    RelativeDestination = 26,
    TeleportedCharacter = 27,
}

/// The PropertyString identifies a specific Character or Object string property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyString {
    Name = 1,
    Title = 2,
    Sex = 3,
    HeritageGroup = 4,
    Template = 5,
    AttackersName = 6,
    Inscription = 7,
    ScribeName = 8,
    VendorsName = 9,
    Fellowship = 10,
    MonarchsName = 11,
    LockCode = 12,
    KeyCode = 13,
    Use = 14,
    ShortDesc = 15,
    LongDesc = 16,
    ActivationTalk = 17,
    UseMessage = 18,
    ItemHeritageGroupRestriction = 19,
    PluralName = 20,
    MonarchsTitle = 21,
    ActivationFailure = 22,
    ScribeAccount = 23,
    TownName = 24,
    CraftsmanName = 25,
    UsePkServerError = 26,
    ScoreCachedText = 27,
    ScoreDefaultEntryFormat = 28,
    ScoreFirstEntryFormat = 29,
    ScoreLastEntryFormat = 30,
    ScoreOnlyEntryFormat = 31,
    ScoreNoEntry = 32,
    Quest = 33,
    GeneratorEvent = 34,
    PatronsTitle = 35,
    HouseOwnerName = 36,
    QuestRestriction = 37,
    AppraisalPortalDestination = 38,
    TinkerName = 39,
    ImbuerName = 40,
    HouseOwnerAccount = 41,
    DisplayName = 42,
    DateOfBirth = 43,
    ThirdPartyApi = 44,
    KillQuest = 45,
    Afk = 46,
    AllegianceName = 47,
    AugmentationAddQuest = 48,
    KillQuest2 = 49,
    KillQuest3 = 50,
    UseSendsSignal = 51,
    GearPlatingName = 52,
}

/// The PropertyFloat identifies a specific Character or Object float property.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PropertyFloat {
    Undef = 0,
    HeartbeatInterval = 1,
    HeartbeatTimestamp = 2,
    HealthRate = 3,
    StaminaRate = 4,
    ManaRate = 5,
    HealthUponResurrection = 6,
    StaminaUponResurrection = 7,
    ManaUponResurrection = 8,
    StartTime = 9,
    StopTime = 10,
    ResetInterval = 11,
    Shade = 12,
    ArmorModVsSlash = 13,
    ArmorModVsPierce = 14,
    ArmorModVsBludgeon = 15,
    ArmorModVsCold = 16,
    ArmorModVsFire = 17,
    ArmorModVsAcid = 18,
    ArmorModVsElectric = 19,
    CombatSpeed = 20,
    WeaponLength = 21,
    DamageVariance = 22,
    CurrentPowerMod = 23,
    AccuracyMod = 24,
    StrengthMod = 25,
    MaximumVelocity = 26,
    RotationSpeed = 27,
    MotionTimestamp = 28,
    WeaponDefense = 29,
    WimpyLevel = 30,
    VisualAwarenessRange = 31,
    AuralAwarenessRange = 32,
    PerceptionLevel = 33,
    PowerupTime = 34,
    MaxChargeDistance = 35,
    ChargeSpeed = 36,
    BuyPrice = 37,
    SellPrice = 38,
    DefaultScale = 39,
    LockpickMod = 40,
    RegenerationInterval = 41,
    RegenerationTimestamp = 42,
    GeneratorRadius = 43,
    TimeToRot = 44,
    DeathTimestamp = 45,
    PkTimestamp = 46,
    VictimTimestamp = 47,
    LoginTimestamp = 48,
    CreationTimestamp = 49,
    MinimumTimeSincePk = 50,
    DeprecatedHousekeepingPriority = 51,
    AbuseLoggingTimestamp = 52,
    LastPortalTeleportTimestamp = 53,
    UseRadius = 54,
    HomeRadius = 55,
    ReleasedTimestamp = 56,
    MinHomeRadius = 57,
    Facing = 58,
    ResetTimestamp = 59,
    LogoffTimestamp = 60,
    EconRecoveryInterval = 61,
    WeaponOffense = 62,
    DamageMod = 63,
    ResistSlash = 64,
    ResistPierce = 65,
    ResistBludgeon = 66,
    ResistFire = 67,
    ResistCold = 68,
    ResistAcid = 69,
    ResistElectric = 70,
    ResistHealthBoost = 71,
    ResistStaminaDrain = 72,
    ResistStaminaBoost = 73,
    ResistManaDrain = 74,
    ResistManaBoost = 75,
    Translucency = 76,
    PhysicsScriptIntensity = 77,
    Friction = 78,
    Elasticity = 79,
    AiUseMagicDelay = 80,
    ItemMinSpellcraftMod = 81,
    ItemMaxSpellcraftMod = 82,
    ItemRankProbability = 83,
    Shade2 = 84,
    Shade3 = 85,
    Shade4 = 86,
    ItemEfficiency = 87,
    ItemManaUpdateTimestamp = 88,
    SpellGestureSpeedMod = 89,
    SpellStanceSpeedMod = 90,
    AllegianceAppraisalTimestamp = 91,
    PowerLevel = 92,
    AccuracyLevel = 93,
    AttackAngle = 94,
    AttackTimestamp = 95,
    CheckpointTimestamp = 96,
    SoldTimestamp = 97,
    UseTimestamp = 98,
    UseLockTimestamp = 99,
    HealkitMod = 100,
    FrozenTimestamp = 101,
    HealthRateMod = 102,
    AllegianceSwearTimestamp = 103,
    ObviousRadarRange = 104,
    HotspotCycleTime = 105,
    HotspotCycleTimeVariance = 106,
    SpamTimestamp = 107,
    SpamRate = 108,
    BondWieldedTreasure = 109,
    BulkMod = 110,
    SizeMod = 111,
    GagTimestamp = 112,
    GeneratorUpdateTimestamp = 113,
    DeathSpamTimestamp = 114,
    DeathSpamRate = 115,
    WildAttackProbability = 116,
    FocusedProbability = 117,
    CrashAndTurnProbability = 118,
    CrashAndTurnRadius = 119,
    CrashAndTurnBias = 120,
    GeneratorInitialDelay = 121,
    AiAcquireHealth = 122,
    AiAcquireStamina = 123,
    AiAcquireMana = 124,
    ResistHealthDrain = 125,
    LifestoneProtectionTimestamp = 126,
    AiCounteractEnchantment = 127,
    AiDispelEnchantment = 128,
    TradeTimestamp = 129,
    AiTargetedDetectionRadius = 130,
    EmotePriority = 131,
    LastTeleportStartTimestamp = 132,
    EventSpamTimestamp = 133,
    EventSpamRate = 134,
    InventoryOffset = 135,
    CriticalMultiplier = 136,
    ManaStoneDestroyChance = 137,
    SlayerDamageBonus = 138,
    AllegianceInfoSpamTimestamp = 139,
    AllegianceInfoSpamRate = 140,
    NextSpellcastTimestamp = 141,
    AppraisalRequestedTimestamp = 142,
    AppraisalHeartbeatDueTimestamp = 143,
    ManaConversionMod = 144,
    LastPkAttackTimestamp = 145,
    FellowshipUpdateTimestamp = 146,
    CriticalFrequency = 147,
    LimboStartTimestamp = 148,
    WeaponMissileDefense = 149,
    WeaponMagicDefense = 150,
    IgnoreShield = 151,
    ElementalDamageMod = 152,
    StartMissileAttackTimestamp = 153,
    LastRareUsedTimestamp = 154,
    IgnoreArmor = 155,
    ProcSpellRate = 156,
    ResistanceModifier = 157,
    AllegianceGagTimestamp = 158,
    AbsorbMagicDamage = 159,
    CachedMaxAbsorbMagicDamage = 160,
    GagDuration = 161,
    AllegianceGagDuration = 162,
    GlobalXpMod = 163,
    HealingModifier = 164,
    ArmorModVsNether = 165,
    ResistNether = 166,
    CooldownDuration = 167,
    WeaponAuraOffense = 168,
    WeaponAuraDefense = 169,
    WeaponAuraElemental = 170,
    WeaponAuraManaConv = 171,
}

/// Chat channels
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Channel {
        pub bits: u32,
}

/// Equipment Set Ids
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EquipmentSet {
    None = 0,
    Test = 1,
    Test2 = 2,
    Unknown3 = 3,
    CarraidasBenediction = 4,
    NobleRelic = 5,
    AncientRelic = 6,
    AlduressaRelic = 7,
    Ninja = 8,
    EmpyreanRings = 9,
    ArmMindHeart = 10,
    ArmorPerfectLight = 11,
    ArmorPerfectLight2 = 12,
    Soldiers = 13,
    Adepts = 14,
    Archers = 15,
    Defenders = 16,
    Tinkers = 17,
    Crafters = 18,
    Hearty = 19,
    Dexterous = 20,
    Wise = 21,
    Swift = 22,
    Hardened = 23,
    Reinforced = 24,
    Interlocking = 25,
    Flameproof = 26,
    Acidproof = 27,
    Coldproof = 28,
    Lightningproof = 29,
    SocietyArmor = 30,
    ColosseumClothing = 31,
    GraveyardClothing = 32,
    OlthoiClothing = 33,
    NoobieArmor = 34,
    AetheriaDefense = 35,
    AetheriaDestruction = 36,
    AetheriaFury = 37,
    AetheriaGrowth = 38,
    AetheriaVigor = 39,
    RareDamageResistance = 40,
    RareDamageBoost = 41,
    OlthoiArmorDRed = 42,
    OlthoiArmorCRat = 43,
    OlthoiArmorCRed = 44,
    OlthoiArmorDRat = 45,
    AlduressaRelicUpgrade = 46,
    AncientRelicUpgrade = 47,
    NobleRelicUpgrade = 48,
    CloakAlchemy = 49,
    CloakArcaneLore = 50,
    CloakArmorTinkering = 51,
    CloakAssessPerson = 52,
    CloakLightWeapons = 53,
    CloakMissileWeapons = 54,
    CloakCooking = 55,
    CloakCreatureEnchantment = 56,
    CloakCrossbow = 57,
    CloakFinesseWeapons = 58,
    CloakDeception = 59,
    CloakFletching = 60,
    CloakHealing = 61,
    CloakItemEnchantment = 62,
    CloakItemTinkering = 63,
    CloakLeadership = 64,
    CloakLifeMagic = 65,
    CloakLoyalty = 66,
    CloakMace = 67,
    CloakMagicDefense = 68,
    CloakMagicItemTinkering = 69,
    CloakManaConversion = 70,
    CloakMeleeDefense = 71,
    CloakMissileDefense = 72,
    CloakSalvaging = 73,
    CloakSpear = 74,
    CloakStaff = 75,
    CloakHeavyWeapons = 76,
    CloakThrownWeapon = 77,
    CloakTwoHandedCombat = 78,
    CloakUnarmedCombat = 79,
    CloakVoidMagic = 80,
    CloakWarMagic = 81,
    CloakWeaponTinkering = 82,
    CloakAssessCreature = 83,
    CloakDirtyFighting = 84,
    CloakDualWield = 85,
    CloakRecklessness = 86,
    CloakShield = 87,
    CloakSneakAttack = 88,
    Ninja_New = 89,
    CloakSummoning = 90,
    ShroudedSoul = 91,
    DarkenedMind = 92,
    CloudedSpirit = 93,
    MinorStingingShroudedSoul = 94,
    MinorSparkingShroudedSoul = 95,
    MinorSmolderingShroudedSoul = 96,
    MinorShiveringShroudedSoul = 97,
    MinorStingingDarkenedMind = 98,
    MinorSparkingDarkenedMind = 99,
    MinorSmolderingDarkenedMind = 100,
    MinorShiveringDarkenedMind = 101,
    MinorStingingCloudedSpirit = 102,
    MinorSparkingCloudedSpirit = 103,
    MinorSmolderingCloudedSpirit = 104,
    MinorShiveringCloudedSpirit = 105,
    MajorStingingShroudedSoul = 106,
    MajorSparkingShroudedSoul = 107,
    MajorSmolderingShroudedSoul = 108,
    MajorShiveringShroudedSoul = 109,
    MajorStingingDarkenedMind = 110,
    MajorSparkingDarkenedMind = 111,
    MajorSmolderingDarkenedMind = 112,
    MajorShiveringDarkenedMind = 113,
    MajorStingingCloudedSpirit = 114,
    MajorSparkingCloudedSpirit = 115,
    MajorSmolderingCloudedSpirit = 116,
    MajorShiveringCloudedSpirit = 117,
    BlackfireStingingShroudedSoul = 118,
    BlackfireSparkingShroudedSoul = 119,
    BlackfireSmolderingShroudedSoul = 120,
    BlackfireShiveringShroudedSoul = 121,
    BlackfireStingingDarkenedMind = 122,
    BlackfireSparkingDarkenedMind = 123,
    BlackfireSmolderingDarkenedMind = 124,
    BlackfireShiveringDarkenedMind = 125,
    BlackfireStingingCloudedSpirit = 126,
    BlackfireSparkingCloudedSpirit = 127,
    BlackfireSmolderingCloudedSpirit = 128,
    BlackfireShiveringCloudedSpirit = 129,
    ShimmeringShadowsSet = 130,
    BrownSocietyLocket = 131,
    YellowSocietyLocket = 132,
    RedSocietyBand = 133,
    GreenSocietyBand = 134,
    PurpleSocietyBand = 135,
    BlueSocietyBand = 136,
    GauntletGarb = 137,
    ParagonMissile = 138,
    ParagonCaster = 139,
    ParagonMelee = 140,
}

/// Radar Color
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum RadarColor {
    Default = 0x00,
    Blue = 0x01,
    Gold = 0x02,
    White = 0x03,
    Purple = 0x04,
    Red = 0x05,
    Pink = 0x06,
    Green = 0x07,
    Yellow = 0x08,
    Cyan = 0x09,
    BrightGreen = 0x10,
}

/// Flags that determine what data is contained in the EnchantmentRegistry
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnchantmentRegistryFlags {
        pub bits: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SpellCategory {
    Undef = 0,
    StrengthRaising = 1,
    StrengthLowering = 2,
    EnduranceRaising = 3,
    EnduranceLowering = 4,
    QuicknessRaising = 5,
    QuicknessLowering = 6,
    CoordinationRaising = 7,
    CoordinationLowering = 8,
    FocusRaising = 9,
    FocusLowering = 10,
    SelfRaising = 11,
    SelfLowering = 12,
    FocusConcentration = 13,
    FocusDisruption = 14,
    FocusBrilliance = 15,
    FocusDullness = 16,
    AxeRaising = 17,
    AxeLowering = 18,
    BowRaising = 19,
    BowLowering = 20,
    CrossbowRaising = 21,
    CrossbowLowering = 22,
    DaggerRaising = 23,
    DaggerLowering = 24,
    MaceRaising = 25,
    MaceLowering = 26,
    SpearRaising = 27,
    SpearLowering = 28,
    StaffRaising = 29,
    StaffLowering = 30,
    SwordRaising = 31,
    SwordLowering = 32,
    ThrownWeaponsRaising = 33,
    ThrownWeaponsLowering = 34,
    UnarmedCombatRaising = 35,
    UnarmedCombatLowering = 36,
    MeleeDefenseRaising = 37,
    MeleeDefenseLowering = 38,
    MissileDefenseRaising = 39,
    MissileDefenseLowering = 40,
    MagicDefenseRaising = 41,
    MagicDefenseLowering = 42,
    CreatureEnchantmentRaising = 43,
    CreatureEnchantmentLowering = 44,
    ItemEnchantmentRaising = 45,
    ItemEnchantmentLowering = 46,
    LifeMagicRaising = 47,
    LifeMagicLowering = 48,
    WarMagicRaising = 49,
    WarMagicLowering = 50,
    ManaConversionRaising = 51,
    ManaConversionLowering = 52,
    ArcaneLoreRaising = 53,
    ArcaneLoreLowering = 54,
    AppraiseArmorRaising = 55,
    AppraiseArmorLowering = 56,
    AppraiseItemRaising = 57,
    AppraiseItemLowering = 58,
    AppraiseMagicItemRaising = 59,
    AppraiseMagicItemLowering = 60,
    AppraiseWeaponRaising = 61,
    AppraiseWeaponLowering = 62,
    AssessMonsterRaising = 63,
    AssessMonsterLowering = 64,
    DeceptionRaising = 65,
    DeceptionLowering = 66,
    HealingRaising = 67,
    HealingLowering = 68,
    JumpRaising = 69,
    JumpLowering = 70,
    LeadershipRaising = 71,
    LeadershipLowering = 72,
    LockpickRaising = 73,
    LockpickLowering = 74,
    LoyaltyRaising = 75,
    LoyaltyLowering = 76,
    RunRaising = 77,
    RunLowering = 78,
    HealthRaising = 79,
    HealthLowering = 80,
    StaminaRaising = 81,
    StaminaLowering = 82,
    ManaRaising = 83,
    ManaLowering = 84,
    ManaRemedy = 85,
    ManaMalediction = 86,
    HealthTransfertocaster = 87,
    HealthTransferfromcaster = 88,
    StaminaTransfertocaster = 89,
    StaminaTransferfromcaster = 90,
    ManaTransfertocaster = 91,
    ManaTransferfromcaster = 92,
    HealthAccelerating = 93,
    HealthDecelerating = 94,
    StaminaAccelerating = 95,
    StaminaDecelerating = 96,
    ManaAccelerating = 97,
    ManaDecelerating = 98,
    VitaeRaising = 99,
    VitaeLowering = 100,
    AcidProtection = 101,
    AcidVulnerability = 102,
    BludgeonProtection = 103,
    BludgeonVulnerability = 104,
    ColdProtection = 105,
    ColdVulnerability = 106,
    ElectricProtection = 107,
    ElectricVulnerability = 108,
    FireProtection = 109,
    FireVulnerability = 110,
    PierceProtection = 111,
    PierceVulnerability = 112,
    SlashProtection = 113,
    SlashVulnerability = 114,
    ArmorRaising = 115,
    ArmorLowering = 116,
    AcidMissile = 117,
    BludgeoningMissile = 118,
    ColdMissile = 119,
    ElectricMissile = 120,
    FireMissile = 121,
    PiercingMissile = 122,
    SlashingMissile = 123,
    AcidSeeker = 124,
    BludgeoningSeeker = 125,
    ColdSeeker = 126,
    ElectricSeeker = 127,
    FireSeeker = 128,
    PiercingSeeker = 129,
    SlashingSeeker = 130,
    AcidBurst = 131,
    BludgeoningBurst = 132,
    ColdBurst = 133,
    ElectricBurst = 134,
    FireBurst = 135,
    PiercingBurst = 136,
    SlashingBurst = 137,
    AcidBlast = 138,
    BludgeoningBlast = 139,
    ColdBlast = 140,
    ElectricBlast = 141,
    FireBlast = 142,
    PiercingBlast = 143,
    SlashingBlast = 144,
    AcidScatter = 145,
    BludgeoningScatter = 146,
    ColdScatter = 147,
    ElectricScatter = 148,
    FireScatter = 149,
    PiercingScatter = 150,
    SlashingScatter = 151,
    AttackModRaising = 152,
    AttackModLowering = 153,
    DamageRaising = 154,
    DamageLowering = 155,
    DefenseModRaising = 156,
    DefenseModLowering = 157,
    WeaponTimeRaising = 158,
    WeaponTimeLowering = 159,
    ArmorValueRaising = 160,
    ArmorValueLowering = 161,
    AcidResistanceRaising = 162,
    AcidResistanceLowering = 163,
    BludgeonResistanceRaising = 164,
    BludgeonResistanceLowering = 165,
    ColdResistanceRaising = 166,
    ColdResistanceLowering = 167,
    ElectricResistanceRaising = 168,
    ElectricResistanceLowering = 169,
    FireResistanceRaising = 170,
    FireResistanceLowering = 171,
    PierceResistanceRaising = 172,
    PierceResistanceLowering = 173,
    SlashResistanceRaising = 174,
    SlashResistanceLowering = 175,
    BludgeoningResistanceRaising = 176,
    BludgeoningResistanceLowering = 177,
    SlashingResistanceRaising = 178,
    SlashingResistanceLowering = 179,
    PiercingResistanceRaising = 180,
    PiercingResistanceLowering = 181,
    ElectricalResistanceRaising = 182,
    ElectricalResistanceLowering = 183,
    FrostResistanceRaising = 184,
    FrostResistanceLowering = 185,
    FlameResistanceRaising = 186,
    FlameResistanceLowering = 187,
    AcidicResistanceRaising = 188,
    AcidicResistanceLowering = 189,
    ArmorLevelRaising = 190,
    ArmorLevelLowering = 191,
    LockpickResistanceRaising = 192,
    LockpickResistanceLowering = 193,
    ManaConversionModLowering = 194,
    ManaConversionModRaising = 195,
    VisionRaising = 196,
    VisionLowering = 197,
    TransparencyRaising = 198,
    TransparencyLowering = 199,
    PortalTie = 200,
    PortalRecall = 201,
    PortalCreation = 202,
    PortalItemCreation = 203,
    Vitae = 204,
    AssessPersonRaising = 205,
    AssessPersonLowering = 206,
    AcidVolley = 207,
    BludgeoningVolley = 208,
    FrostVolley = 209,
    LightningVolley = 210,
    FlameVolley = 211,
    ForceVolley = 212,
    BladeVolley = 213,
    PortalSending = 214,
    LifestoneSending = 215,
    CookingRaising = 216,
    CookingLowering = 217,
    FletchingRaising = 218,
    FletchingLowering = 219,
    AlchemyLowering = 220,
    AlchemyRaising = 221,
    AcidRing = 222,
    BludgeoningRing = 223,
    ColdRing = 224,
    ElectricRing = 225,
    FireRing = 226,
    PiercingRing = 227,
    SlashingRing = 228,
    AcidWall = 229,
    BludgeoningWall = 230,
    ColdWall = 231,
    ElectricWall = 232,
    FireWall = 233,
    PiercingWall = 234,
    SlashingWall = 235,
    AcidStrike = 236,
    BludgeoningStrike = 237,
    ColdStrike = 238,
    ElectricStrike = 239,
    FireStrike = 240,
    PiercingStrike = 241,
    SlashingStrike = 242,
    AcidStreak = 243,
    BludgeoningStreak = 244,
    ColdStreak = 245,
    ElectricStreak = 246,
    FireStreak = 247,
    PiercingStreak = 248,
    SlashingStreak = 249,
    Dispel = 250,
    CreatureMysticRaising = 251,
    CreatureMysticLowering = 252,
    ItemMysticRaising = 253,
    ItemMysticLowering = 254,
    WarMysticRaising = 255,
    WarMysticLowering = 256,
    HealthRestoring = 257,
    HealthDepleting = 258,
    ManaRestoring = 259,
    ManaDepleting = 260,
    StrengthIncrease = 261,
    StrengthDecrease = 262,
    EnduranceIncrease = 263,
    EnduranceDecrease = 264,
    QuicknessIncrease = 265,
    QuicknessDecrease = 266,
    CoordinationIncrease = 267,
    CoordinationDecrease = 268,
    FocusIncrease = 269,
    FocusDecrease = 270,
    SelfIncrease = 271,
    SelfDecrease = 272,
    GreatVitalityRaising = 273,
    PoorVitalityLowering = 274,
    GreatVigorRaising = 275,
    PoorVigorLowering = 276,
    GreaterIntellectRaising = 277,
    LessorIntellectLowering = 278,
    LifeGiverRaising = 279,
    LifeTakerLowering = 280,
    StaminaGiverRaising = 281,
    StaminaTakerLowering = 282,
    ManaGiverRaising = 283,
    ManaTakerLowering = 284,
    AcidWardProtection = 285,
    AcidWardVulnerability = 286,
    FireWardProtection = 287,
    FireWardVulnerability = 288,
    ColdWardProtection = 289,
    ColdWardVulnerability = 290,
    ElectricWardProtection = 291,
    ElectricWardVulnerability = 292,
    LeadershipObedienceRaising = 293,
    LeadershipObedienceLowering = 294,
    MeleeDefenseShelterRaising = 295,
    MeleeDefenseShelterLowering = 296,
    MissileDefenseShelterRaising = 297,
    MissileDefenseShelterLowering = 298,
    MagicDefenseShelterRaising = 299,
    MagicDefenseShelterLowering = 300,
    HuntersAcumenRaising = 301,
    HuntersAcumenLowering = 302,
    StillWaterRaising = 303,
    StillWaterLowering = 304,
    StrengthofEarthRaising = 305,
    StrengthofEarthLowering = 306,
    TorrentRaising = 307,
    TorrentLowering = 308,
    GrowthRaising = 309,
    GrowthLowering = 310,
    CascadeAxeRaising = 311,
    CascadeAxeLowering = 312,
    CascadeDaggerRaising = 313,
    CascadeDaggerLowering = 314,
    CascadeMaceRaising = 315,
    CascadeMaceLowering = 316,
    CascadeSpearRaising = 317,
    CascadeSpearLowering = 318,
    CascadeStaffRaising = 319,
    CascadeStaffLowering = 320,
    StoneCliffsRaising = 321,
    StoneCliffsLowering = 322,
    MaxDamageRaising = 323,
    MaxDamageLowering = 324,
    BowDamageRaising = 325,
    BowDamageLowering = 326,
    BowRangeRaising = 327,
    BowRangeLowering = 328,
    ExtraDefenseModRaising = 329,
    ExtraDefenseModLowering = 330,
    ExtraBowSkillRaising = 331,
    ExtraBowSkillLowering = 332,
    ExtraAlchemySkillRaising = 333,
    ExtraAlchemySkillLowering = 334,
    ExtraArcaneLoreSkillRaising = 335,
    ExtraArcaneLoreSkillLowering = 336,
    ExtraAppraiseArmorSkillRaising = 337,
    ExtraAppraiseArmorSkillLowering = 338,
    ExtraCookingSkillRaising = 339,
    ExtraCookingSkillLowering = 340,
    ExtraCrossbowSkillRaising = 341,
    ExtraCrossbowSkillLowering = 342,
    ExtraDeceptionSkillRaising = 343,
    ExtraDeceptionSkillLowering = 344,
    ExtraLoyaltySkillRaising = 345,
    ExtraLoyaltySkillLowering = 346,
    ExtraFletchingSkillRaising = 347,
    ExtraFletchingSkillLowering = 348,
    ExtraHealingSkillRaising = 349,
    ExtraHealingSkillLowering = 350,
    ExtraMeleeDefenseSkillRaising = 351,
    ExtraMeleeDefenseSkillLowering = 352,
    ExtraAppraiseItemSkillRaising = 353,
    ExtraAppraiseItemSkillLowering = 354,
    ExtraJumpingSkillRaising = 355,
    ExtraJumpingSkillLowering = 356,
    ExtraLifeMagicSkillRaising = 357,
    ExtraLifeMagicSkillLowering = 358,
    ExtraLockpickSkillRaising = 359,
    ExtraLockpickSkillLowering = 360,
    ExtraAppraiseMagicItemSkillRaising = 361,
    ExtraAppraiseMagicItemSkillLowering = 362,
    ExtraManaConversionSkillRaising = 363,
    ExtraManaConversionSkillLowering = 364,
    ExtraAssessCreatureSkillRaising = 365,
    ExtraAssessCreatureSkillLowering = 366,
    ExtraAssessPersonSkillRaising = 367,
    ExtraAssessPersonSkillLowering = 368,
    ExtraRunSkillRaising = 369,
    ExtraRunSkillLowering = 370,
    ExtraSwordSkillRaising = 371,
    ExtraSwordSkillLowering = 372,
    ExtraThrownWeaponsSkillRaising = 373,
    ExtraThrownWeaponsSkillLowering = 374,
    ExtraUnarmedCombatSkillRaising = 375,
    ExtraUnarmedCombatSkillLowering = 376,
    ExtraAppraiseWeaponSkillRaising = 377,
    ExtraAppraiseWeaponSkillLowering = 378,
    ArmorIncrease = 379,
    ArmorDecrease = 380,
    ExtraAcidResistanceRaising = 381,
    ExtraAcidResistanceLowering = 382,
    ExtraBludgeonResistanceRaising = 383,
    ExtraBludgeonResistanceLowering = 384,
    ExtraFireResistanceRaising = 385,
    ExtraFireResistanceLowering = 386,
    ExtraColdResistanceRaising = 387,
    ExtraColdResistanceLowering = 388,
    ExtraAttackModRaising = 389,
    ExtraAttackModLowering = 390,
    ExtraArmorValueRaising = 391,
    ExtraArmorValueLowering = 392,
    ExtraPierceResistanceRaising = 393,
    ExtraPierceResistanceLowering = 394,
    ExtraSlashResistanceRaising = 395,
    ExtraSlashResistanceLowering = 396,
    ExtraElectricResistanceRaising = 397,
    ExtraElectricResistanceLowering = 398,
    ExtraWeaponTimeRaising = 399,
    ExtraWeaponTimeLowering = 400,
    BludgeonWardProtection = 401,
    BludgeonWardVulnerability = 402,
    SlashWardProtection = 403,
    SlashWardVulnerability = 404,
    PierceWardProtection = 405,
    PierceWardVulnerability = 406,
    StaminaRestoring = 407,
    StaminaDepleting = 408,
    Fireworks = 409,
    HealthDivide = 410,
    StaminaDivide = 411,
    ManaDivide = 412,
    CoordinationIncrease2 = 413,
    StrengthIncrease2 = 414,
    FocusIncrease2 = 415,
    EnduranceIncrease2 = 416,
    SelfIncrease2 = 417,
    MeleeDefenseMultiply = 418,
    MissileDefenseMultiply = 419,
    MagicDefenseMultiply = 420,
    AttributesDecrease = 421,
    LifeGiverRaising2 = 422,
    ItemEnchantmentRaising2 = 423,
    SkillsDecrease = 424,
    ExtraManaConversionBonus = 425,
    WarMysticRaising2 = 426,
    WarMysticLowering2 = 427,
    MagicDefenseShelterRaising2 = 428,
    ExtraLifeMagicSkillRaising2 = 429,
    CreatureMysticRaising2 = 430,
    ItemMysticRaising2 = 431,
    ManaRaising2 = 432,
    SelfRaising2 = 433,
    CreatureEnchantmentRaising2 = 434,
    SalvagingRaising = 435,
    ExtraSalvagingRaising = 436,
    ExtraSalvagingRaising2 = 437,
    CascadeAxeRaising2 = 438,
    ExtraBowSkillRaising2 = 439,
    ExtraThrownWeaponsSkillRaising2 = 440,
    ExtraCrossbowSkillRaising2 = 441,
    CascadeDaggerRaising2 = 442,
    CascadeMaceRaising2 = 443,
    ExtraUnarmedCombatSkillRaising2 = 444,
    CascadeSpearRaising2 = 445,
    CascadeStaffRaising2 = 446,
    ExtraSwordSkillRaising2 = 447,
    AcidProtectionRare = 448,
    AcidResistanceRaisingRare = 449,
    AlchemyRaisingRare = 450,
    AppraisalResistanceLoweringRare = 451,
    AppraiseArmorRaisingRare = 452,
    AppraiseItemRaisingRare = 453,
    AppraiseMagicItemRaisingRare = 454,
    AppraiseWeaponRaisingRare = 455,
    ArcaneLoreRaisingRare = 456,
    ArmorRaisingRare = 457,
    ArmorValueRaisingRare = 458,
    AssessMonsterRaisingRare = 459,
    AssessPersonRaisingRare = 460,
    AttackModRaisingRare = 461,
    AxeRaisingRare = 462,
    BludgeonProtectionRare = 463,
    BludgeonResistanceRaisingRare = 464,
    BowRaisingRare = 465,
    ColdProtectionRare = 466,
    ColdResistanceRaisingRare = 467,
    CookingRaisingRare = 468,
    CoordinationRaisingRare = 469,
    CreatureEnchantmentRaisingRare = 470,
    CrossbowRaisingRare = 471,
    DaggerRaisingRare = 472,
    DamageRaisingRare = 473,
    DeceptionRaisingRare = 474,
    DefenseModRaisingRare = 475,
    ElectricProtectionRare = 476,
    ElectricResistanceRaisingRare = 477,
    EnduranceRaisingRare = 478,
    FireProtectionRare = 479,
    FireResistanceRaisingRare = 480,
    FletchingRaisingRare = 481,
    FocusRaisingRare = 482,
    HealingRaisingRare = 483,
    HealthAcceleratingRare = 484,
    ItemEnchantmentRaisingRare = 485,
    JumpRaisingRare = 486,
    LeadershipRaisingRare = 487,
    LifeMagicRaisingRare = 488,
    LockpickRaisingRare = 489,
    LoyaltyRaisingRare = 490,
    MaceRaisingRare = 491,
    MagicDefenseRaisingRare = 492,
    ManaAcceleratingRare = 493,
    ManaConversionRaisingRare = 494,
    MeleeDefenseRaisingRare = 495,
    MissileDefenseRaisingRare = 496,
    PierceProtectionRare = 497,
    PierceResistanceRaisingRare = 498,
    QuicknessRaisingRare = 499,
    RunRaisingRare = 500,
    SelfRaisingRare = 501,
    SlashProtectionRare = 502,
    SlashResistanceRaisingRare = 503,
    SpearRaisingRare = 504,
    StaffRaisingRare = 505,
    StaminaAcceleratingRare = 506,
    StrengthRaisingRare = 507,
    SwordRaisingRare = 508,
    ThrownWeaponsRaisingRare = 509,
    UnarmedCombatRaisingRare = 510,
    WarMagicRaisingRare = 511,
    WeaponTimeRaisingRare = 512,
    ArmorIncreaseInkyArmor = 513,
    MagicDefenseShelterRaisingFiun = 514,
    ExtraRunSkillRaisingFiun = 515,
    ExtraManaConversionSkillRaisingFiun = 516,
    AttributesIncreaseCantrip1 = 517,
    ExtraMeleeDefenseSkillRaising2 = 518,
    ACTDPurchaseRewardSpell = 519,
    ACTDPurchaseRewardSpellHealth = 520,
    SaltAshAttackModRaising = 521,
    QuicknessIncrease2 = 522,
    ExtraAlchemySkillRaising2 = 523,
    ExtraCookingSkillRaising2 = 524,
    ExtraFletchingSkillRaising2 = 525,
    ExtraLockpickSkillRaising2 = 526,
    MucorManaWell = 527,
    StaminaRestoring2 = 528,
    AllegianceRaising = 529,
    HealthDoT = 530,
    HealthDoTSecondary = 531,
    HealthDoTTertiary = 532,
    HealthHoT = 533,
    HealthHoTSecondary = 534,
    HealthHoTTertiary = 535,
    HealthDivideSecondary = 536,
    HealthDivideTertiary = 537,
    SetSwordRaising = 538,
    SetAxeRaising = 539,
    SetDaggerRaising = 540,
    SetMaceRaising = 541,
    SetSpearRaising = 542,
    SetStaffRaising = 543,
    SetUnarmedRaising = 544,
    SetBowRaising = 545,
    SetCrossbowRaising = 546,
    SetThrownRaising = 547,
    SetItemEnchantmentRaising = 548,
    SetCreatureEnchantmentRaising = 549,
    SetWarMagicRaising = 550,
    SetLifeMagicRaising = 551,
    SetMeleeDefenseRaising = 552,
    SetMissileDefenseRaising = 553,
    SetMagicDefenseRaising = 554,
    SetStaminaAccelerating = 555,
    SetCookingRaising = 556,
    SetFletchingRaising = 557,
    SetLockpickRaising = 558,
    SetAlchemyRaising = 559,
    SetSalvagingRaising = 560,
    SetArmorExpertiseRaising = 561,
    SetWeaponExpertiseRaising = 562,
    SetItemTinkeringRaising = 563,
    SetMagicItemExpertiseRaising = 564,
    SetLoyaltyRaising = 565,
    SetStrengthRaising = 566,
    SetEnduranceRaising = 567,
    SetCoordinationRaising = 568,
    SetQuicknessRaising = 569,
    SetFocusRaising = 570,
    SetWillpowerRaising = 571,
    SetHealthRaising = 572,
    SetStaminaRaising = 573,
    SetManaRaising = 574,
    SetSprintRaising = 575,
    SetJumpingRaising = 576,
    SetSlashResistanceRaising = 577,
    SetBludgeonResistanceRaising = 578,
    SetPierceResistanceRaising = 579,
    SetFlameResistanceRaising = 580,
    SetAcidResistanceRaising = 581,
    SetFrostResistanceRaising = 582,
    SetLightningResistanceRaising = 583,
    CraftingLockPickRaising = 584,
    CraftingFletchingRaising = 585,
    CraftingCookingRaising = 586,
    CraftingAlchemyRaising = 587,
    CraftingArmorTinkeringRaising = 588,
    CraftingWeaponTinkeringRaising = 589,
    CraftingMagicTinkeringRaising = 590,
    CraftingItemTinkeringRaising = 591,
    SkillPercentAlchemyRaising = 592,
    TwoHandedRaising = 593,
    TwoHandedLowering = 594,
    ExtraTwoHandedSkillRaising = 595,
    ExtraTwoHandedSkillLowering = 596,
    ExtraTwoHandedSkillRaising2 = 597,
    TwoHandedRaisingRare = 598,
    SetTwoHandedRaising = 599,
    GearCraftRaising = 600,
    GearCraftLowering = 601,
    ExtraGearCraftSkillRaising = 602,
    ExtraGearCraftSkillLowering = 603,
    ExtraGearCraftSkillRaising2 = 604,
    GearCraftRaisingRare = 605,
    SetGearCraftRaising = 606,
    LoyaltyManaRaising = 607,
    LoyaltyStaminaRaising = 608,
    LeadershipHealthRaising = 609,
    TrinketDamageRaising = 610,
    TrinketDamageLowering = 611,
    TrinketHealthRaising = 612,
    TrinketStaminaRaising = 613,
    TrinketManaRaising = 614,
    TrinketXPRaising = 615,
    DeceptionArcaneLoreRaising = 616,
    HealOverTimeRaising = 617,
    DamageOverTimeRaising = 618,
    HealingResistRatingRaising = 619,
    AetheriaDamageRatingRaising = 620,
    AetheriaDamageReductionRaising = 621,
    AetheriaHealthRaising = 623,
    AetheriaStaminaRaising = 624,
    AetheriaManaRaising = 625,
    AetheriaCriticalDamageRaising = 626,
    AetheriaHealingAmplificationRaising = 627,
    AetheriaProcDamageRatingRaising = 628,
    AetheriaProcDamageReductionRaising = 629,
    AetheriaProcHealthOverTimeRaising = 630,
    AetheriaProcDamageOverTimeRaising = 631,
    AetheriaProcHealingReductionRaising = 632,
    RareDamageRatingRaising = 633,
    RareDamageReductionRatingRaising = 634,
    AetheriaEnduranceRaising = 635,
    NetherDamageOverTimeRaising = 636,
    NetherDamageOverTimeRaising2 = 637,
    NetherDamageOverTimeRaising3 = 638,
    NetherStreak = 639,
    NetherMissile = 640,
    NetherRing = 641,
    NetherDamageRatingLowering = 642,
    NetherDamageHealingReductionRaising = 643,
    VoidMagicLowering = 644,
    VoidMagicRaising = 645,
    VoidMysticRaising = 646,
    SetVoidMagicRaising = 647,
    VoidMagicRaisingRare = 648,
    VoidMysticRaising2 = 649,
    LuminanceDamageRatingRaising = 650,
    LuminanceDamageReductionRaising = 651,
    LuminanceHealthRaising = 652,
    AetheriaCriticalReductionRaising = 653,
    ExtraMissileDefenseSkillRaising = 654,
    ExtraMissileDefenseSkillLowering = 655,
    ExtraMissileDefenseSkillRaising2 = 656,
    AetheriaHealthResistanceRaising = 657,
    AetheriaDotResistanceRaising = 658,
    CloakSkillRaising = 659,
    CloakAllSkillRaising = 660,
    CloakMagicDefenseLowering = 661,
    CloakMeleeDefenseLowering = 662,
    CloakMissileDefenseLowering = 663,
    DirtyFightingLowering = 664,
    DirtyFightingRaising = 665,
    ExtraDirtyFightingRaising = 666,
    DualWieldLowering = 667,
    DualWieldRaising = 668,
    ExtraDualWieldRaising = 669,
    RecklessnessLowering = 670,
    RecklessnessRaising = 671,
    ExtraRecklessnessRaising = 672,
    ShieldLowering = 673,
    ShieldRaising = 674,
    ExtraShieldRaising = 675,
    SneakAttackLowering = 676,
    SneakAttackRaising = 677,
    ExtraSneakAttackRaising = 678,
    RareDirtyFightingRaising = 679,
    RareDualWieldRaising = 680,
    RareRecklessnessRaising = 681,
    RareShieldRaising = 682,
    RareSneakAttackRaising = 683,
    DFAttackSkillDebuff = 684,
    DFBleedDamage = 685,
    DFDefenseSkillDebuff = 686,
    DFHealingDebuff = 687,
    SetDirtyFightingRaising = 688,
    SetDualWieldRaising = 689,
    SetRecklessnessRaising = 690,
    SetShieldRaising = 691,
    SetSneakAttackRaising = 692,
    LifeGiverMhoire = 693,
    RareDamageRatingRaising2 = 694,
    SpellDamageRaising = 695,
    SummoningRaising = 696,
    SummoningLowering = 697,
    ExtraSummoningSkillRaising = 698,
    SetSummoningRaising = 699,
    ParagonEnduranceRaising = 704,
    ParagonManaRaising = 705,
    ParagonStaminaRaising = 706,
    ParagonDirtyFightingRaising = 707,
    ParagonDualWieldRaising = 708,
    ParagonRecklessnessRaising = 709,
    ParagonSneakAttackRaising = 710,
    ParagonDamageRatingRaising = 711,
    ParagonDamageReductionRatingRaising = 712,
    ParagonCriticalDamageRatingRaising = 713,
    ParagonCriticalDamageReductionRatingRaising = 714,
    ParagonAxeRaising = 715,
    ParagonDaggerRaising = 716,
    ParagonSwordRaising = 717,
    ParagonWarMagicRaising = 718,
    ParagonLifeMagicRaising = 719,
    ParagonVoidMagicRaising = 720,
    ParagonBowRaising = 721,
    ParagonStrengthRaising = 722,
    ParagonCoordinationRaising = 723,
    ParagonQuicknessRaising = 724,
    ParagonFocusRaising = 725,
    ParagonWillpowerRaising = 726,
    ParagonTwoHandedRaising = 727,
    GauntletDamageReductionRatingRaising = 728,
    GauntletDamageRatingRaising = 729,
    GauntletHealingRatingRaising = 730,
    GauntletVitalityRaising = 731,
    GauntletCriticalDamageRatingRaising = 732,
    GauntletCriticalDamageReductionRatingRaising = 733,
}

/// Heritage of a player
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum HeritageGroup {
    Invalid = 0x00,
    Aluvian = 0x01,
    Gharundim = 0x02,
    Sho = 0x03,
    Viamontian = 0x04,
    Shadowbound = 0x05,
    Gearknight = 0x06,
    Tumerok = 0x07,
    Lugian = 0x08,
    Empyrean = 0x09,
    Penumbraen = 0x0A,
    Undead = 0x0B,
    Olthoi = 0x0C,
    OlthoiAcid = 0x0D,
}

/// the type of highlight (outline) applied to the object's icon
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IconHighlight {
        pub bits: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CombatUse {
    None = 0x00,
    Melee = 0x01,
    Missile = 0x02,
    Ammo = 0x03,
    Shield = 0x04,
    TwoHanded = 0x05,
}

/// the type of wieldable item this is
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum WieldType {
    Invalid = 0x00000000,
    MeleeWeapon = 0x00000001,
    Armor = 0x00000002,
    Clothing = 0x00000004,
    Jewelry = 0x00000008,
}

/// Chat channel type, for turbine chat
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChatType {
    Undef = 0x00000000,
    Allegiance = 0x00000001,
    General = 0x00000002,
    Trade = 0x00000003,
    LFG = 0x00000004,
    Roleplay = 0x00000005,
    Society = 0x00000006,
    SocietyCelHan = 0x00000007,
    SocietyEldWeb = 0x00000008,
    SocietyRadBlo = 0x00000009,
    Olthoi = 0x0000000a,
}

/// The ChatDisplayMask identifies that types of chat that are displayed in each chat window. 
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ChatDisplayMask {
    Gameplay = 0x03912021,
    Mandatory = 0x0000c302,
    AreaChat = 0x00001004,
    Tells = 0x00000018,
    Combat = 0x00600040,
    Magic = 0x00020080,
    Allegiance = 0x00040c00,
    Fellowship = 0x00080000,
    Errors = 0x04000000,
    TradeChannel = 0x10000000,
    LFGChannel = 0x20000000,
    RoleplayChannel = 0x40000000,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum EnchantmentMask {
    Multiplicative = 0x03912021,
    Additive = 0x03912021,
    Vitae = 0x03912021,
    Cooldown = 0x03912021,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EnchantmentTypeFlags {
        pub bits: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentLocation {
    None = 0x0000000,
    RightHand = 0x0000001,
    LeftHand = 0x0000002,
    Shield = 0x0000003,
    Belt = 0x0000004,
    Quiver = 0x0000005,
    Hearldry = 0x0000006,
    Mouth = 0x0000007,
    LeftWeapon = 0x0000008,
    LeftUnarmed = 0x0000009,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Placement {
    Default = 0x0000000,
    RightHandCombat = 0x0000001,
    RightHandNonCombat = 0x0000002,
    LeftHand = 0x0000003,
    Belt = 0x0000004,
    Quiver = 0x0000005,
    Shield = 0x0000006,
    LeftWeapon = 0x0000007,
    LeftUnarmed = 0x0000008,
    SpecialCrowssbowBolt = 0x0000033,
    MissileFlight = 0x0000034,
    Resting = 0x0000065,
    Other = 0x0000066,
    Hook = 0x0000067,
    Random1 = 0x0000079,
    Random2 = 0x000007a,
    Random3 = 0x000007b,
    Random4 = 0x000007c,
    Random5 = 0x000007d,
    Random6 = 0x000007e,
    Random7 = 0x000007f,
    Random8 = 0x0000080,
    Random9 = 0x0000081,
    Random10 = 0x0000082,
    XXXUnknownA = 0x0000000A,
    XXXUnknownF = 0x0000000F,
    XXXUnknown14 = 0x00000014,
    XXXUnknown1E = 0x0000001E,
    XXXUnknown20 = 0x00000020,
    XXXUnknown3C = 0x0000003C,
    XXXUnknown69 = 0x00000069,
    XXXUnknown6A = 0x0000006A,
    XXXUnknown63 = 0x00000063,
    XXXUnknown68 = 0x00000068,
    XXXUnknown78 = 0x00000078,
    XXXUnknown84 = 0x00000084,
    XXXUnknownF0 = 0x000000F0,
    XXXUnknown3F2 = 0x000003F2,
}

#[allow(non_camel_case_types)]
pub type byte = u8;

#[allow(non_camel_case_types)]
pub type short = i16;

#[allow(non_camel_case_types)]
pub type ushort = u16;

#[allow(non_camel_case_types)]
pub type int = i32;

#[allow(non_camel_case_types)]
pub type uint = u32;

#[allow(non_camel_case_types)]
pub type long = i64;

#[allow(non_camel_case_types)]
pub type ulong = u64;

#[allow(non_camel_case_types)]
pub type float = f32;

#[allow(non_camel_case_types)]
pub type double = f64;

#[allow(non_camel_case_types)]
pub type string = String;

#[allow(non_camel_case_types)]
pub type WString = String;

#[allow(non_camel_case_types)]
pub type WORD = u16;

#[allow(non_camel_case_types)]
pub type DWORD = u32;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedWORD {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedDWORD {}

#[allow(non_camel_case_types)]
pub type ObjectId = u32;

#[allow(non_camel_case_types)]
pub type LandcellId = u32;

#[allow(non_camel_case_types)]
pub type SpellId = u16;

#[allow(non_camel_case_types)]
pub type DataId = PackedDWORD;

// Full spell Id combining the spell id with the spell layer.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LayeredSpellId {
        #[serde(rename = "Id")]
        id: SpellId,
        #[serde(rename = "Layer")]
        layer: u16
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector3 {
        #[serde(rename = "X")]
        x: f32,
        #[serde(rename = "Y")]
        y: f32,
        #[serde(rename = "Z")]
        z: f32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
        #[serde(rename = "W")]
        w: f32,
        #[serde(rename = "X")]
        x: f32,
        #[serde(rename = "Y")]
        y: f32,
        #[serde(rename = "Z")]
        z: f32
}

// Landcell location, without orientation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Origin {
        #[serde(rename = "Landcell")]
        landcell: LandcellId,
        #[serde(rename = "Location")]
        location: Vector3
}

// Landcell location, including orientation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Position {
        #[serde(rename = "Landcell")]
        landcell: LandcellId,
        #[serde(rename = "Frame")]
        frame: Frame
}

// A the location and orientation of an object within a landcell
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Frame {
        #[serde(rename = "Origin")]
        origin: Vector3,
        #[serde(rename = "Orientation")]
        orientation: Quaternion
}

// Optional header data when PacketHeaderFlags includes ServerSwitch
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServerSwitchHeader {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Type")]
        type_: ServerSwitchType
}

// Optional header data when PacketHeaderFlags includes CICMDCommand
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CICMDCommandHeader {
        #[serde(rename = "Command")]
        command: u32,
        #[serde(rename = "Parameter")]
        parameter: u32
}

// Optional header data when PacketHeaderFlags includes Flow
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FlowHeader {
        #[serde(rename = "Bytes")]
        bytes: u32,
        #[serde(rename = "Interval")]
        interval: u16
}

// Optional header data when PacketHeaderFlags includes LogonServerAddr
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SocketAddress {
        #[serde(rename = "Family")]
        family: i16,
        #[serde(rename = "Port")]
        port: u16,
        #[serde(rename = "Address")]
        address: u32,
        #[serde(rename = "Empty")]
        empty: u64
}

// Optional header data when PacketHeaderFlags includes LoginRequest
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "AuthType")]
pub enum LoginRequestHeader {
    #[serde(rename = "0x00000002")]
    Type00000002 {
        #[serde(rename = "ClientVersion")]
        client_version: String,
        #[serde(rename = "Length")]
        length: u32,
        #[serde(rename = "Flags")]
        flags: AuthFlags,
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "AccountToLoginAs")]
        account_to_login_as: String,
        #[serde(rename = "Password")]
        password: String,
    },
    #[serde(rename = "0x40000002")]
    Type40000002 {
        #[serde(rename = "ClientVersion")]
        client_version: String,
        #[serde(rename = "Length")]
        length: u32,
        #[serde(rename = "Flags")]
        flags: AuthFlags,
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "AccountToLoginAs")]
        account_to_login_as: String,
        #[serde(rename = "GlsTicket")]
        gls_ticket: String,
    },
}

// Optional header data when PacketHeaderFlags includes Referral
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReferralHeader {
        #[serde(rename = "Cookie")]
        cookie: u64,
        #[serde(rename = "Address")]
        address: SocketAddress,
        #[serde(rename = "IdServer")]
        id_server: u16,
        #[serde(rename = "Unknown")]
        unknown: DWORD
}

// Optional header data when PacketHeaderFlags includes ConnectRequest
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectRequestHeader {
        #[serde(rename = "ServerTime")]
        server_time: f64,
        #[serde(rename = "Cookie")]
        cookie: u64,
        #[serde(rename = "NetID")]
        net_id: i32,
        #[serde(rename = "OutgoingSeed")]
        outgoing_seed: u32,
        #[serde(rename = "IncomingSeed")]
        incoming_seed: u32,
        #[serde(rename = "Unknown")]
        unknown: DWORD
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NetError {
        #[serde(rename = "StringId")]
        string_id: DataId,
        #[serde(rename = "TableId")]
        table_id: DataId
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EchoResponseHeader {
        #[serde(rename = "LocalTime")]
        local_time: f32,
        #[serde(rename = "HoldingTime")]
        holding_time: f32
}

// A collection of property tables.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ACBaseQualities {
        #[serde(rename = "Flags")]
        flags: ACBaseQualitiesFlags,
        #[serde(rename = "WeenieType")]
        weenie_type: WeenieType,
        #[serde(rename = "IntProperties")]
        int_properties: PackableHashTable,
        #[serde(rename = "Int64Properties")]
        int64_properties: PackableHashTable,
        #[serde(rename = "BoolProperties")]
        bool_properties: PackableHashTable,
        #[serde(rename = "FloatProperties")]
        float_properties: PackableHashTable,
        #[serde(rename = "StringProperties")]
        string_properties: PackableHashTable,
        #[serde(rename = "DataProperties")]
        data_properties: PackableHashTable,
        #[serde(rename = "InstanceProperties")]
        instance_properties: PackableHashTable,
        #[serde(rename = "PositionProperties")]
        position_properties: PackableHashTable
}

// The ACQualities structure contains character property lists.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ACQualities {
        #[serde(rename = "Flags")]
        flags: ACQualitiesFlags,
        #[serde(rename = "HasHealth")]
        has_health: bool,
        #[serde(rename = "Attributes")]
        attributes: AttributeCache,
        #[serde(rename = "Skills")]
        skills: PackableHashTable,
        #[serde(rename = "Body")]
        body: Body,
        #[serde(rename = "SpellBook")]
        spell_book: PackableHashTable,
        #[serde(rename = "Enchantments")]
        enchantments: EnchantmentRegistry,
        #[serde(rename = "EventFilter")]
        event_filter: EventFilter,
        #[serde(rename = "Emotes")]
        emotes: EmoteTable,
        #[serde(rename = "CreationProfile")]
        creation_profile: PackableList,
        #[serde(rename = "PageData")]
        page_data: PageDataList,
        #[serde(rename = "Generators")]
        generators: GeneratorTable,
        #[serde(rename = "GeneratorRegistry")]
        generator_registry: GeneratorRegistry,
        #[serde(rename = "GeneratorQueue")]
        generator_queue: GeneratorQueue
}

// The AttributeCache structure contains information about a character attributes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AttributeCache {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Strength")]
        strength: AttributeInfo,
        #[serde(rename = "Endurance")]
        endurance: AttributeInfo,
        #[serde(rename = "Quickness")]
        quickness: AttributeInfo,
        #[serde(rename = "Coordination")]
        coordination: AttributeInfo,
        #[serde(rename = "Focus")]
        focus: AttributeInfo,
        #[serde(rename = "Self")]
        self_: AttributeInfo,
        #[serde(rename = "Health")]
        health: SecondaryAttributeInfo,
        #[serde(rename = "Stamina")]
        stamina: SecondaryAttributeInfo,
        #[serde(rename = "Mana")]
        mana: SecondaryAttributeInfo
}

// The Attribute structure contains information about a character attribute.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AttributeInfo {
        #[serde(rename = "PointsRaised")]
        points_raised: u32,
        #[serde(rename = "InnatePoints")]
        innate_points: u32,
        #[serde(rename = "ExperienceSpent")]
        experience_spent: u32
}

// The SecondaryAttribute structure contains information about a character vital.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SecondaryAttributeInfo {
        #[serde(rename = "Attribute")]
        attribute: AttributeInfo,
        #[serde(rename = "Current")]
        current: u32
}

// The Skill structure contains information about a character skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Skill {
        #[serde(rename = "PointsRaised")]
        points_raised: u16,
        #[serde(rename = "AdjustPP")]
        adjust_pp: u16,
        #[serde(rename = "TrainingLevel")]
        training_level: SkillAdvancementClass,
        #[serde(rename = "ExperienceSpent")]
        experience_spent: u32,
        #[serde(rename = "InnatePoints")]
        innate_points: u32,
        #[serde(rename = "ResistanceOfLastCheck")]
        resistance_of_last_check: u32,
        #[serde(rename = "LastUsedTime")]
        last_used_time: f64
}

// Contains body part table
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Body {
        #[serde(rename = "BodyParts")]
        body_parts: PackableHashTable
}

// Information on individual body parts. (Needs to be confirmed if this was used in prod)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BodyPart {
        #[serde(rename = "HasBPSD")]
        has_bpsd: i32,
        #[serde(rename = "DamageType")]
        damage_type: DamageType,
        #[serde(rename = "DamageVal")]
        damage_val: i32,
        #[serde(rename = "DamageVar")]
        damage_var: i32,
        #[serde(rename = "ArmorCache")]
        armor_cache: ArmorCache,
        #[serde(rename = "BH")]
        bh: i32,
        #[serde(rename = "BPSD")]
        bpsd: BodyPartSelectionData
}

// Information on armor levels
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ArmorCache {
        #[serde(rename = "BaseArmor")]
        base_armor: i32,
        #[serde(rename = "ArmorVsSlash")]
        armor_vs_slash: i32,
        #[serde(rename = "ArmorVsPierce")]
        armor_vs_pierce: i32,
        #[serde(rename = "ArmorVsBludgeon")]
        armor_vs_bludgeon: i32,
        #[serde(rename = "ArmorVsCold")]
        armor_vs_cold: i32,
        #[serde(rename = "ArmorVsFire")]
        armor_vs_fire: i32,
        #[serde(rename = "ArmorVsAcid")]
        armor_vs_acid: i32,
        #[serde(rename = "ArmorVsElectric")]
        armor_vs_electric: i32,
        #[serde(rename = "ArmorVsNether")]
        armor_vs_nether: i32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BodyPartSelectionData {
        #[serde(rename = "HLF")]
        hlf: i32,
        #[serde(rename = "MLF")]
        mlf: i32,
        #[serde(rename = "LLF")]
        llf: i32,
        #[serde(rename = "HRF")]
        hrf: i32,
        #[serde(rename = "MRF")]
        mrf: i32,
        #[serde(rename = "LRF")]
        lrf: i32,
        #[serde(rename = "HLB")]
        hlb: i32,
        #[serde(rename = "MLB")]
        mlb: i32,
        #[serde(rename = "LLB")]
        llb: i32,
        #[serde(rename = "HRB")]
        hrb: i32,
        #[serde(rename = "MRB")]
        mrb: i32,
        #[serde(rename = "LRB")]
        lrb: i32
}

// Contains information related to the spells in effect on the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnchantmentRegistry {
        #[serde(rename = "Flags")]
        flags: EnchantmentRegistryFlags,
        #[serde(rename = "LifeSpells")]
        life_spells: PackableList,
        #[serde(rename = "CreatureSpells")]
        creature_spells: PackableList,
        #[serde(rename = "Vitae")]
        vitae: Enchantment,
        #[serde(rename = "Cooldowns")]
        cooldowns: PackableList
}

// Information on stat modification
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatMod {
        #[serde(rename = "Type")]
        type_: EnchantmentTypeFlags,
        #[serde(rename = "Key")]
        key: u32,
        #[serde(rename = "Value")]
        value: f32
}

// Contains a list of events to filter? Unknown what this does currently.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EventFilter {
        #[serde(rename = "Events")]
        events: PackableList
}

// Contains a list of emotes for NPCs? Unknown what this does currently.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmoteTable {
        #[serde(rename = "Emotes")]
        emotes: PackableHashTable
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmoteSetList {
        #[serde(rename = "Emotes")]
        emotes: PackableList
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Category")]
pub enum EmoteSet {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x06")]
    Type01 {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "ClassId")]
        class_id: u32,
    },
    #[serde(rename = "0x02")]
    Type02 {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "VendorType")]
        vendor_type: u32,
    },
    #[serde(rename = "0x05")]
    Type05 {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "Style")]
        style: u32,
        #[serde(rename = "Substyle")]
        substyle: u32,
    },
    #[serde(rename = "0x0C")]
    #[serde(alias = "0x0D")]
    #[serde(alias = "0x16")]
    #[serde(alias = "0x17")]
    #[serde(alias = "0x1B")]
    #[serde(alias = "0x1C")]
    #[serde(alias = "0x1D")]
    #[serde(alias = "0x1E")]
    #[serde(alias = "0x1F")]
    #[serde(alias = "0x20")]
    #[serde(alias = "0x21")]
    #[serde(alias = "0x22")]
    #[serde(alias = "0x23")]
    #[serde(alias = "0x24")]
    #[serde(alias = "0x25")]
    #[serde(alias = "0x26")]
    Type0C {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "Quest")]
        quest: String,
    },
    #[serde(rename = "0x0F")]
    Type0F {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "Emotes")]
        emotes: PackableList,
        #[serde(rename = "MinHealth")]
        min_health: f32,
        #[serde(rename = "MaxHealth")]
        max_health: f32,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum Emote {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x08")]
    #[serde(alias = "0x0A")]
    #[serde(alias = "0x0D")]
    #[serde(alias = "0x10")]
    #[serde(alias = "0x11")]
    #[serde(alias = "0x12")]
    #[serde(alias = "0x14")]
    #[serde(alias = "0x15")]
    #[serde(alias = "0x16")]
    #[serde(alias = "0x17")]
    #[serde(alias = "0x18")]
    #[serde(alias = "0x19")]
    #[serde(alias = "0x1A")]
    #[serde(alias = "0x1F")]
    #[serde(alias = "0x33")]
    #[serde(alias = "0x3A")]
    #[serde(alias = "0x3C")]
    #[serde(alias = "0x3D")]
    #[serde(alias = "0x40")]
    #[serde(alias = "0x41")]
    #[serde(alias = "0x43")]
    #[serde(alias = "0x44")]
    #[serde(alias = "0x4F")]
    #[serde(alias = "0x50")]
    #[serde(alias = "0x51")]
    #[serde(alias = "0x53")]
    #[serde(alias = "0x58")]
    #[serde(alias = "0x79")]
    Type01 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
    },
    #[serde(rename = "0x13")]
    #[serde(alias = "0x1B")]
    #[serde(alias = "0x49")]
    #[serde(alias = "0xE")]
    Type13 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "SpellId")]
        spell_id: u32,
    },
    #[serde(rename = "0x1C")]
    #[serde(alias = "0x1D")]
    Type1C {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount")]
        amount: u32,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x1E")]
    #[serde(alias = "0x3B")]
    #[serde(alias = "0x47")]
    #[serde(alias = "0x52")]
    Type1E {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Min")]
        min: u32,
        #[serde(rename = "Max")]
        max: u32,
    },
    #[serde(rename = "0x2")]
    #[serde(alias = "0x3E")]
    Type2 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount64")]
        amount64: u64,
        #[serde(rename = "HeroXP64")]
        hero_xp64: u64,
    },
    #[serde(rename = "0x20")]
    #[serde(alias = "0x21")]
    #[serde(alias = "0x46")]
    #[serde(alias = "0x54")]
    #[serde(alias = "0x55")]
    #[serde(alias = "0x56")]
    #[serde(alias = "0x59")]
    #[serde(alias = "0x66")]
    #[serde(alias = "0x67")]
    #[serde(alias = "0x68")]
    #[serde(alias = "0x69")]
    #[serde(alias = "0x6A")]
    #[serde(alias = "0x6B")]
    #[serde(alias = "0x6C")]
    #[serde(alias = "0x6D")]
    Type20 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Amount")]
        amount: u32,
    },
    #[serde(rename = "0x22")]
    #[serde(alias = "0x2F")]
    #[serde(alias = "0x30")]
    #[serde(alias = "0x5A")]
    #[serde(alias = "0x6F")]
    #[serde(alias = "0x77")]
    #[serde(alias = "0x78")]
    Type22 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount")]
        amount: u32,
    },
    #[serde(rename = "0x23")]
    #[serde(alias = "0x2D")]
    #[serde(alias = "0x2E")]
    Type23 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x24")]
    #[serde(alias = "0x27")]
    #[serde(alias = "0x28")]
    #[serde(alias = "0x29")]
    #[serde(alias = "0x2A")]
    #[serde(alias = "0x2B")]
    #[serde(alias = "0x2C")]
    Type24 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Min")]
        min: u32,
        #[serde(rename = "Max")]
        max: u32,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x25")]
    Type25 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "FMin")]
        f_min: f64,
        #[serde(rename = "FMax")]
        f_max: f64,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x26")]
    #[serde(alias = "0x4B")]
    Type26 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "TestString")]
        test_string: String,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x3")]
    #[serde(alias = "0x4A")]
    Type3 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "CProfile")]
        c_profile: CreationProfile,
    },
    #[serde(rename = "0x31")]
    Type31 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Percent")]
        percent: f64,
        #[serde(rename = "Min64")]
        min64: u64,
        #[serde(rename = "Max64")]
        max64: u64,
    },
    #[serde(rename = "0x32")]
    Type32 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Stat")]
        stat: u32,
        #[serde(rename = "Percent")]
        percent: f64,
        #[serde(rename = "Min")]
        min: u32,
        #[serde(rename = "Max")]
        max: u32,
        #[serde(rename = "Display")]
        display: bool,
    },
    #[serde(rename = "0x34")]
    #[serde(alias = "0x5")]
    Type34 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Motion")]
        motion: u32,
    },
    #[serde(rename = "0x35")]
    #[serde(alias = "0x36")]
    #[serde(alias = "0x37")]
    #[serde(alias = "0x45")]
    Type35 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Stat")]
        stat: u32,
        #[serde(rename = "Amount")]
        amount: u32,
    },
    #[serde(rename = "0x38")]
    Type38 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "WealthRating")]
        wealth_rating: i32,
        #[serde(rename = "TreasureClass")]
        treasure_class: i32,
        #[serde(rename = "TreasureType")]
        treasure_type: i32,
    },
    #[serde(rename = "0x3F")]
    #[serde(alias = "0x63")]
    #[serde(alias = "0x64")]
    Type3F {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Position")]
        position: Position,
    },
    #[serde(rename = "0x4")]
    #[serde(alias = "0x57")]
    #[serde(alias = "0x6")]
    #[serde(alias = "0xB")]
    Type4 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Frame")]
        frame: Frame,
    },
    #[serde(rename = "0x4C")]
    Type4C {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        msg: String,
        #[serde(rename = "CProfile")]
        c_profile: CreationProfile,
    },
    #[serde(rename = "0x6E")]
    #[serde(alias = "0x73")]
    Type6E {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x7")]
    Type7 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "PhysicsScript")]
        physics_script: u32,
    },
    #[serde(rename = "0x70")]
    #[serde(alias = "0x71")]
    Type70 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Amount64")]
        amount64: u64,
    },
    #[serde(rename = "0x72")]
    Type72 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "Min64")]
        min64: u64,
        #[serde(rename = "Max64")]
        max64: u64,
        #[serde(rename = "Stat")]
        stat: u32,
    },
    #[serde(rename = "0x76")]
    Type76 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Stat")]
        stat: u32,
        #[serde(rename = "Percent")]
        percent: f64,
    },
    #[serde(rename = "0x9")]
    Type9 {
        #[serde(rename = "Delay")]
        delay: f32,
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Sound")]
        sound: u32,
    },
}

// Set information about an item for creation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreationProfile {
        #[serde(rename = "WeenieClassId")]
        weenie_class_id: u32,
        #[serde(rename = "Palette")]
        palette: u32,
        #[serde(rename = "Shade")]
        shade: f32,
        #[serde(rename = "Destination")]
        destination: u32,
        #[serde(rename = "StackSize")]
        stack_size: i32,
        #[serde(rename = "TryToBond")]
        try_to_bond: bool
}

// List of pages in a book
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PageDataList {
        #[serde(rename = "MaxNumPages")]
        max_num_pages: u32,
        #[serde(rename = "MaxNumCharsPerPage")]
        max_num_chars_per_page: u32,
        #[serde(rename = "Pages")]
        pages: PackableList
}

// Blob fragment data used to contruct message data. These can be spread across multiple packets
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlobFragments {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Id")]
        id: u32,
        #[serde(rename = "Count")]
        count: u16,
        #[serde(rename = "Size")]
        size: u16,
        #[serde(rename = "Index")]
        index: u16,
        #[serde(rename = "Group")]
        group: FragmentGroup
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratorTable {
        #[serde(rename = "Generators")]
        generators: PackableList
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorProfile {
        #[serde(rename = "Probability")]
        probability: f32,
        #[serde(rename = "TypeId")]
        type_id: u32,
        #[serde(rename = "Delay")]
        delay: f64,
        #[serde(rename = "InitCreate")]
        init_create: u32,
        #[serde(rename = "MaxNum")]
        max_num: u32,
        #[serde(rename = "WhenCreate")]
        when_create: u32,
        #[serde(rename = "WhereCreate")]
        where_create: u32,
        #[serde(rename = "StackSize")]
        stack_size: u32,
        #[serde(rename = "Ptid")]
        ptid: u32,
        #[serde(rename = "Shade")]
        shade: f32,
        #[serde(rename = "PosVal")]
        pos_val: Position,
        #[serde(rename = "Slot")]
        slot: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRegistry {
        #[serde(rename = "Registry")]
        registry: PackableHashTable
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorRegistryNode {
        #[serde(rename = "WcidOrType")]
        wcid_or_type: u32,
        #[serde(rename = "Ts")]
        ts: f64,
        #[serde(rename = "TreasureType")]
        treasure_type: u32,
        #[serde(rename = "Slot")]
        slot: u32,
        #[serde(rename = "Checkpointed")]
        checkpointed: u32,
        #[serde(rename = "Shop")]
        shop: u32,
        #[serde(rename = "Amount")]
        amount: u32
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueue {
        #[serde(rename = "Queue")]
        queue: PackableList
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneratorQueueNode {
        #[serde(rename = "Slot")]
        slot: u32,
        #[serde(rename = "When")]
        when: f64
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "TitleSource")]
pub enum WindowProperty {
    #[serde(rename = "0x00")]
    Type00 {
        #[serde(rename = "Key_a")]
        key_a: u32,
        #[serde(rename = "Unknown_1b")]
        unknown_1b: u32,
        #[serde(rename = "Unknown_1c")]
        unknown_1c: u16,
        #[serde(rename = "Unknown_d")]
        unknown_d: u32,
        #[serde(rename = "Value_d")]
        value_d: u8,
        #[serde(rename = "Unknown_e")]
        unknown_e: u32,
        #[serde(rename = "Value_e")]
        value_e: u32,
        #[serde(rename = "Unknown_f")]
        unknown_f: u32,
        #[serde(rename = "Value_f")]
        value_f: u32,
        #[serde(rename = "Unknown_h")]
        unknown_h: u32,
        #[serde(rename = "Value_h")]
        value_h: u32,
        #[serde(rename = "Unknown_i")]
        unknown_i: u32,
        #[serde(rename = "Value_i")]
        value_i: u32,
        #[serde(rename = "Unknown_j")]
        unknown_j: u32,
        #[serde(rename = "Value_j")]
        value_j: u64,
        #[serde(rename = "StringId")]
        string_id: u32,
        #[serde(rename = "FileId")]
        file_id: u32,
    },
    #[serde(rename = "0x01")]
    Type01 {
        #[serde(rename = "Key_a")]
        key_a: u32,
        #[serde(rename = "Unknown_1b")]
        unknown_1b: u32,
        #[serde(rename = "Unknown_1c")]
        unknown_1c: u16,
        #[serde(rename = "Unknown_d")]
        unknown_d: u32,
        #[serde(rename = "Value_d")]
        value_d: u8,
        #[serde(rename = "Unknown_e")]
        unknown_e: u32,
        #[serde(rename = "Value_e")]
        value_e: u32,
        #[serde(rename = "Unknown_f")]
        unknown_f: u32,
        #[serde(rename = "Value_f")]
        value_f: u32,
        #[serde(rename = "Unknown_h")]
        unknown_h: u32,
        #[serde(rename = "Value_h")]
        value_h: u32,
        #[serde(rename = "Unknown_i")]
        unknown_i: u32,
        #[serde(rename = "Value_i")]
        value_i: u32,
        #[serde(rename = "Unknown_j")]
        unknown_j: u32,
        #[serde(rename = "Value_j")]
        value_j: u64,
        #[serde(rename = "Value_a")]
        value_a: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type_a")]
pub enum WindowOption {
    #[serde(rename = "0x1000008b")]
    Type1000008B {
        #[serde(rename = "Unknown_b")]
        unknown_b: u8,
        #[serde(rename = "PropertyCount")]
        property_count: u8,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum OptionProperty {
    #[serde(rename = "0x10000080")]
    Type10000080 {
        unknown_l: u32,
        #[serde(rename = "inactiveOpacity")]
        inactive_opacity: f32,
    },
    #[serde(rename = "0x10000081")]
    Type10000081 {
        unknown_k: u32,
        #[serde(rename = "activeOpacity")]
        active_opacity: f32,
    },
    #[serde(rename = "0x1000008c")]
    Type1000008C {
        #[serde(rename = "Unknown_a")]
        unknown_a: u32,
        #[serde(rename = "WindowOptions")]
        window_options: PackableList,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameplayOptions {
        #[serde(rename = "Size")]
        size: u32,
        #[serde(rename = "Unknown200_2")]
        unknown200_2: u8,
        #[serde(rename = "OptionPropertyCount")]
        option_property_count: u8
}

// The PlayerModule structure contains character options.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlayerModule {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Options")]
        options: CharacterOptions1,
        #[serde(rename = "Shortcuts")]
        shortcuts: PackableList,
        #[serde(rename = "Tab1Spells")]
        tab1_spells: PackableList,
        #[serde(rename = "Tab2Spells")]
        tab2_spells: PackableList,
        #[serde(rename = "Tab3Spells")]
        tab3_spells: PackableList,
        #[serde(rename = "Tab4Spells")]
        tab4_spells: PackableList,
        #[serde(rename = "Tab5Spells")]
        tab5_spells: PackableList,
        #[serde(rename = "Tab6Spells")]
        tab6_spells: PackableList,
        #[serde(rename = "Tab7Spells")]
        tab7_spells: PackableList,
        #[serde(rename = "Tab8Spells")]
        tab8_spells: PackableList,
        #[serde(rename = "FillComps")]
        fill_comps: PackableHashTable,
        #[serde(rename = "SpellBookFilters")]
        spell_book_filters: u32,
        #[serde(rename = "OptionFlags")]
        option_flags: u32,
        #[serde(rename = "Unknown100_1")]
        unknown100_1: u32,
        #[serde(rename = "OptionStrings")]
        option_strings: PackableHashTable,
        #[serde(rename = "GameplayOptions")]
        gameplay_options: GameplayOptions
}

// Set of shortcuts
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShortCutManager {
        #[serde(rename = "Shortcuts")]
        shortcuts: PackableList
}

// Shortcut
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ShortCutData {
        #[serde(rename = "Index")]
        index: u32,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// List of spells in spell tab
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SpellTab {
        #[serde(rename = "Spells")]
        spells: PackableList
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContentProfile {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerType")]
        container_type: ContainerProperties
}

// Set of inventory items
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InventoryPlacement {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Location")]
        location: EquipMask,
        #[serde(rename = "Priority")]
        priority: CoverageMask
}

// Allegience information
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AllegianceProfile {
        #[serde(rename = "TotalMembers")]
        total_members: u32,
        #[serde(rename = "TotalVassals")]
        total_vassals: u32,
        #[serde(rename = "Hierarchy")]
        hierarchy: AllegianceHierarchy
}

// Allegience record
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AllegianceRecord {
        #[serde(rename = "TreeParent")]
        tree_parent: ObjectId,
        #[serde(rename = "AllegianceData")]
        allegiance_data: AllegianceData
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FriendData {
        #[serde(rename = "FriendId")]
        friend_id: ObjectId,
        #[serde(rename = "Online")]
        online: bool,
        #[serde(rename = "AppearOffline")]
        appear_offline: bool,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "OutFriends")]
        out_friends: PackableList,
        #[serde(rename = "InFriends")]
        in_friends: PackableList
}

// Data related to an item, namely the amount and description
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "PwdType")]
pub enum ItemProfile {
    #[serde(rename = "-1")]
    TypeNeg1 {
        #[serde(rename = "PackedAmount")]
        packed_amount: u32,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "WeenieDescription")]
        weenie_description: PublicWeenieDesc,
    },
    #[serde(rename = "1")]
    Type1 {
        #[serde(rename = "PackedAmount")]
        packed_amount: u32,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "OldWeenieDescription")]
        old_weenie_description: OldPublicWeenieDesc,
    },
}

// The PublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicWeenieDesc {
        #[serde(rename = "Header")]
        header: u32,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "WeenieClassId")]
        weenie_class_id: PackedDWORD,
        #[serde(rename = "Icon")]
        icon: PackedDWORD,
        #[serde(rename = "Type")]
        type_: ItemType,
        #[serde(rename = "Behavior")]
        behavior: ObjectDescriptionFlag,
        #[serde(rename = "Header2")]
        header2: u32,
        #[serde(rename = "PluralName")]
        plural_name: String,
        #[serde(rename = "ItemsCapacity")]
        items_capacity: u8,
        #[serde(rename = "ContainerCapacity")]
        container_capacity: u8,
        #[serde(rename = "AmmunitionType")]
        ammunition_type: AmmoType,
        #[serde(rename = "Value")]
        value: u32,
        #[serde(rename = "Useability")]
        useability: Usable,
        #[serde(rename = "UseRadius")]
        use_radius: f32,
        #[serde(rename = "TargetType")]
        target_type: ItemType,
        #[serde(rename = "Effects")]
        effects: IconHighlight,
        #[serde(rename = "CombatUse")]
        combat_use: WieldType,
        #[serde(rename = "Structure")]
        structure: u16,
        #[serde(rename = "MaxStructure")]
        max_structure: u16,
        #[serde(rename = "StackSize")]
        stack_size: u16,
        #[serde(rename = "MaxStackSize")]
        max_stack_size: u16,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "WielderId")]
        wielder_id: ObjectId,
        #[serde(rename = "ValidSlots")]
        valid_slots: EquipMask,
        #[serde(rename = "Slot")]
        slot: EquipMask,
        #[serde(rename = "Priority")]
        priority: CoverageMask,
        #[serde(rename = "BlipColor")]
        blip_color: RadarColor,
        #[serde(rename = "RadarEnum")]
        radar_enum: RadarBehavior,
        #[serde(rename = "PhysicsScript")]
        physics_script: u16,
        #[serde(rename = "Workmanship")]
        workmanship: f32,
        #[serde(rename = "Burden")]
        burden: u16,
        #[serde(rename = "SpellId")]
        spell_id: SpellId,
        #[serde(rename = "OwnerId")]
        owner_id: ObjectId,
        #[serde(rename = "Restrictions")]
        restrictions: RestrictionDB,
        #[serde(rename = "HookItemTypes")]
        hook_item_types: HookType,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "HookType")]
        hook_type: HookType,
        #[serde(rename = "IconOverlay")]
        icon_overlay: PackedDWORD,
        #[serde(rename = "IconUnderlay")]
        icon_underlay: PackedDWORD,
        #[serde(rename = "Material")]
        material: MaterialType,
        #[serde(rename = "CooldownId")]
        cooldown_id: u32,
        #[serde(rename = "CooldownDuration")]
        cooldown_duration: u64,
        #[serde(rename = "PetOwnerId")]
        pet_owner_id: ObjectId
}

// The RestrictionDB contains the access control list for a dwelling object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RestrictionDB {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "Permissions")]
        permissions: PHashTable
}

// The OldPublicWeenieDesc structure defines an object's game behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OldPublicWeenieDesc {
        #[serde(rename = "Header")]
        header: u32,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "WeenieClassId")]
        weenie_class_id: PackedDWORD,
        #[serde(rename = "Icon")]
        icon: PackedDWORD,
        #[serde(rename = "Type")]
        type_: ItemType,
        #[serde(rename = "Bitfield")]
        bitfield: ObjectDescriptionFlag,
        #[serde(rename = "PluralName")]
        plural_name: String,
        #[serde(rename = "ItemsCapacity")]
        items_capacity: u8,
        #[serde(rename = "ContainerCapacity")]
        container_capacity: u8,
        #[serde(rename = "Value")]
        value: u32,
        #[serde(rename = "Useability")]
        useability: Usable,
        #[serde(rename = "UseRadius")]
        use_radius: f32,
        #[serde(rename = "tTargetType")]
        t_target_type: ItemType,
        #[serde(rename = "Effects")]
        effects: IconHighlight,
        #[serde(rename = "AmmunitionType")]
        ammunition_type: AmmoType,
        #[serde(rename = "CombatUse")]
        combat_use: WieldType,
        #[serde(rename = "Structure")]
        structure: u16,
        #[serde(rename = "MaxStructure")]
        max_structure: u16,
        #[serde(rename = "StackSize")]
        stack_size: u16,
        #[serde(rename = "MaxStackSize")]
        max_stack_size: u16,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "WielderId")]
        wielder_id: ObjectId,
        #[serde(rename = "ValidSlots")]
        valid_slots: EquipMask,
        #[serde(rename = "Slots")]
        slots: EquipMask,
        #[serde(rename = "Priority")]
        priority: CoverageMask,
        #[serde(rename = "BlipColor")]
        blip_color: RadarColor,
        #[serde(rename = "RadarEnum")]
        radar_enum: RadarBehavior,
        #[serde(rename = "ObviousDistance")]
        obvious_distance: f32,
        #[serde(rename = "Vndwcid")]
        vndwcid: u16,
        #[serde(rename = "SpellId")]
        spell_id: SpellId,
        #[serde(rename = "HouseOwnerId")]
        house_owner_id: ObjectId,
        #[serde(rename = "PhysicsScript")]
        physics_script: u16,
        #[serde(rename = "Restrictions")]
        restrictions: RestrictionDB,
        #[serde(rename = "HookType")]
        hook_type: HookType,
        #[serde(rename = "HookItemTypes")]
        hook_item_types: HookType,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "IconOverlay")]
        icon_overlay: PackedDWORD,
        #[serde(rename = "Material")]
        material: MaterialType
}

// Information related to a secure trade.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade {
        #[serde(rename = "PartnerId")]
        partner_id: ObjectId,
        #[serde(rename = "Sequence")]
        sequence: u64,
        #[serde(rename = "Status")]
        status: u32,
        #[serde(rename = "InitiatorId")]
        initiator_id: ObjectId,
        #[serde(rename = "Accepted")]
        accepted: bool,
        #[serde(rename = "PartnerAccepted")]
        partner_accepted: bool
}

// A jump with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JumpPack {
        #[serde(rename = "Extent")]
        extent: f32,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16
}

// A set of data related to changing states with sequences
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct MoveToStatePack {
        #[serde(rename = "RawMotionState")]
        raw_motion_state: RawMotionState,
        #[serde(rename = "Position")]
        position: Position,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16,
        #[serde(rename = "Contact")]
        contact: u8
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackedMotionCommand {
        #[serde(rename = "CommandId")]
        command_id: Command,
        #[serde(rename = "PackedSequence")]
        packed_sequence: u16,
        #[serde(rename = "Speed")]
        speed: f32
}

// Data related to the movement of the object sent from a client
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RawMotionState {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "CurrentHoldkey")]
        current_holdkey: HoldKey,
        #[serde(rename = "CurrentStyle")]
        current_style: StanceMode,
        #[serde(rename = "ForwardCommand")]
        forward_command: MovementCommand,
        #[serde(rename = "ForwardHoldkey")]
        forward_holdkey: HoldKey,
        #[serde(rename = "ForwardSpeed")]
        forward_speed: f32,
        #[serde(rename = "SidestepCommand")]
        sidestep_command: MovementCommand,
        #[serde(rename = "SidestepHoldkey")]
        sidestep_holdkey: HoldKey,
        #[serde(rename = "SidestepSpeed")]
        sidestep_speed: f32,
        #[serde(rename = "TurnCommand")]
        turn_command: MovementCommand,
        #[serde(rename = "TurnHoldkey")]
        turn_holdkey: u32,
        #[serde(rename = "TurnSpeed")]
        turn_speed: f32
}

// An autonomous position with sequences
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AutonomousPositionPack {
        #[serde(rename = "Position")]
        position: Position,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16,
        #[serde(rename = "Contact")]
        contact: u8
}

// A position with sequences
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PositionPack {
        #[serde(rename = "Flags")]
        flags: PositionFlags,
        #[serde(rename = "Origin")]
        origin: Origin,
        #[serde(rename = "WQuat")]
        w_quat: f32,
        #[serde(rename = "XQuat")]
        x_quat: f32,
        #[serde(rename = "YQuat")]
        y_quat: f32,
        #[serde(rename = "ZQuat")]
        z_quat: f32,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "PlacementId")]
        placement_id: u32,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectPositionSequence")]
        object_position_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16
}

// Data related to the movement and animation of the object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "MovementType")]
pub enum MovementData {
    #[serde(rename = "0x0000")]
    Type0000 {
        #[serde(rename = "ObjectMovementSequence")]
        object_movement_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "Autonomous")]
        autonomous: u16,
        #[serde(rename = "OptionFlags")]
        option_flags: MovementOption,
        #[serde(rename = "Stance")]
        stance: StanceMode,
        #[serde(rename = "State")]
        state: InterpertedMotionState,
        #[serde(rename = "StickyObject")]
        sticky_object: ObjectId,
    },
    #[serde(rename = "0x0006")]
    Type0006 {
        #[serde(rename = "ObjectMovementSequence")]
        object_movement_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "Autonomous")]
        autonomous: u16,
        #[serde(rename = "OptionFlags")]
        option_flags: MovementOption,
        #[serde(rename = "Stance")]
        stance: StanceMode,
        #[serde(rename = "Target")]
        target: ObjectId,
        #[serde(rename = "Origin")]
        origin: Origin,
        #[serde(rename = "MoveToParams")]
        move_to_params: MoveToMovementParameters,
        #[serde(rename = "MyRunRate")]
        my_run_rate: f32,
    },
    #[serde(rename = "0x0007")]
    Type0007 {
        #[serde(rename = "ObjectMovementSequence")]
        object_movement_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "Autonomous")]
        autonomous: u16,
        #[serde(rename = "OptionFlags")]
        option_flags: MovementOption,
        #[serde(rename = "Stance")]
        stance: StanceMode,
        #[serde(rename = "Origin")]
        origin: Origin,
        #[serde(rename = "MoveToParams")]
        move_to_params: MoveToMovementParameters,
        #[serde(rename = "MyRunRate")]
        my_run_rate: f32,
    },
    #[serde(rename = "0x0008")]
    Type0008 {
        #[serde(rename = "ObjectMovementSequence")]
        object_movement_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "Autonomous")]
        autonomous: u16,
        #[serde(rename = "OptionFlags")]
        option_flags: MovementOption,
        #[serde(rename = "Stance")]
        stance: StanceMode,
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "DesiredHeading")]
        desired_heading: f32,
        #[serde(rename = "TurnToParams")]
        turn_to_params: TurnToMovementParameters,
    },
    #[serde(rename = "0x0009")]
    Type0009 {
        #[serde(rename = "ObjectMovementSequence")]
        object_movement_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "Autonomous")]
        autonomous: u16,
        #[serde(rename = "OptionFlags")]
        option_flags: MovementOption,
        #[serde(rename = "Stance")]
        stance: StanceMode,
        #[serde(rename = "TurnToParams")]
        turn_to_params: TurnToMovementParameters,
    },
}

// Contains information for animations and general free motion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InterpertedMotionState {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "CurrentStyle")]
        current_style: StanceMode,
        #[serde(rename = "ForwardCommand")]
        forward_command: MovementCommand,
        #[serde(rename = "SidestepCommand")]
        sidestep_command: MovementCommand,
        #[serde(rename = "TurnCommand")]
        turn_command: MovementCommand,
        #[serde(rename = "ForwardSpeed")]
        forward_speed: f32,
        #[serde(rename = "SidestepSpeed")]
        sidestep_speed: f32,
        #[serde(rename = "TurnSpeed")]
        turn_speed: f32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDDRevision {
        #[serde(rename = "IdDatFile")]
        id_dat_file: u64,
        #[serde(rename = "Iteration")]
        iteration: u32,
        #[serde(rename = "IdsToDownload")]
        ids_to_download: PackableList,
        #[serde(rename = "IdsToPurge")]
        ids_to_purge: PackableList
}

// Set of movement parameters required for a MoveTo movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MoveToMovementParameters {
        #[serde(rename = "Bitmember")]
        bitmember: u32,
        #[serde(rename = "DistanceToObject")]
        distance_to_object: f32,
        #[serde(rename = "MinDistance")]
        min_distance: f32,
        #[serde(rename = "FailDistance")]
        fail_distance: f32,
        #[serde(rename = "AnimationSpeed")]
        animation_speed: f32,
        #[serde(rename = "WalkRunThreshold")]
        walk_run_threshold: f32,
        #[serde(rename = "DesiredHeading")]
        desired_heading: f32
}

// Set of movement parameters required for a TurnTo motion
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TurnToMovementParameters {
        #[serde(rename = "Bitmember")]
        bitmember: u32,
        #[serde(rename = "AnimationSpeed")]
        animation_speed: f32,
        #[serde(rename = "DesiredHeading")]
        desired_heading: f32
}

// Contains data for a subpalette
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Subpalette {
        #[serde(rename = "Palette")]
        palette: DataId,
        #[serde(rename = "Offset")]
        offset: u8,
        #[serde(rename = "NumColors")]
        num_colors: u8
}

// Contains data for texture map changes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TextureMapChange {
        #[serde(rename = "PartIndex")]
        part_index: u8,
        #[serde(rename = "OldTexId")]
        old_tex_id: DataId,
        #[serde(rename = "NewTexId")]
        new_tex_id: DataId
}

// Contains data for animation part changes
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AnimPartChange {
        #[serde(rename = "PartIndex")]
        part_index: u8,
        #[serde(rename = "PartId")]
        part_id: DataId
}

// Data for a character creation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CharGenResult {
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "One")]
        one: u32,
        #[serde(rename = "HeritageGroup")]
        heritage_group: HeritageGroup,
        #[serde(rename = "Gender")]
        gender: Gender,
        #[serde(rename = "EyesStrip")]
        eyes_strip: u32,
        #[serde(rename = "NoseStrip")]
        nose_strip: u32,
        #[serde(rename = "MouthStrip")]
        mouth_strip: u32,
        #[serde(rename = "HairColor")]
        hair_color: u32,
        #[serde(rename = "EyeColor")]
        eye_color: u32,
        #[serde(rename = "HairStyle")]
        hair_style: u32,
        #[serde(rename = "HeadgearStyle")]
        headgear_style: u32,
        #[serde(rename = "HeadgearColor")]
        headgear_color: u32,
        #[serde(rename = "ShirtStyle")]
        shirt_style: u32,
        #[serde(rename = "ShirtColor")]
        shirt_color: u32,
        #[serde(rename = "TrousersStyle")]
        trousers_style: u32,
        #[serde(rename = "TrousersColor")]
        trousers_color: u32,
        #[serde(rename = "FootwearStyle")]
        footwear_style: u32,
        #[serde(rename = "FootwearColor")]
        footwear_color: u32,
        #[serde(rename = "SkinShade")]
        skin_shade: u64,
        #[serde(rename = "HairShade")]
        hair_shade: u64,
        #[serde(rename = "HeadgearShade")]
        headgear_shade: u64,
        #[serde(rename = "ShirtShade")]
        shirt_shade: u64,
        #[serde(rename = "TrousersShade")]
        trousers_shade: u64,
        #[serde(rename = "TootwearShade")]
        tootwear_shade: u64,
        #[serde(rename = "TemplateNum")]
        template_num: u32,
        #[serde(rename = "Strength")]
        strength: u32,
        #[serde(rename = "Endurance")]
        endurance: u32,
        #[serde(rename = "Coordination")]
        coordination: u32,
        #[serde(rename = "Quickness")]
        quickness: u32,
        #[serde(rename = "Focus")]
        focus: u32,
        #[serde(rename = "Self")]
        self_: u32,
        #[serde(rename = "Slot")]
        slot: u32,
        #[serde(rename = "ClassId")]
        class_id: u32,
        #[serde(rename = "Skills")]
        skills: PackableList,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "StartArea")]
        start_area: u32,
        #[serde(rename = "IsAdmin")]
        is_admin: u32,
        #[serde(rename = "IsEnvoy")]
        is_envoy: u32,
        #[serde(rename = "Validation")]
        validation: u32
}

// Basic information for a character used at the Login screen
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CharacterIdentity {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "SecondsGreyedOut")]
        seconds_greyed_out: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EquipLocation {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// The PhysicsDesc structure defines an object's physical behavior.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PhysicsDesc {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "State")]
        state: PhysicsState,
        #[serde(rename = "MovementBuffer")]
        movement_buffer: PackableList,
        #[serde(rename = "Autonomous")]
        autonomous: bool,
        #[serde(rename = "AnimationFrame")]
        animation_frame: u32,
        #[serde(rename = "Position")]
        position: Position,
        #[serde(rename = "MotionId")]
        motion_id: DataId,
        #[serde(rename = "SoundId")]
        sound_id: DataId,
        #[serde(rename = "PhysicsScriptId")]
        physics_script_id: DataId,
        #[serde(rename = "SetupId")]
        setup_id: DataId,
        #[serde(rename = "ParentId")]
        parent_id: ObjectId,
        #[serde(rename = "ParentLocation")]
        parent_location: ParentLocation,
        #[serde(rename = "Children")]
        children: PackableList,
        #[serde(rename = "Scale")]
        scale: f32,
        #[serde(rename = "Friction")]
        friction: f32,
        #[serde(rename = "Elasticity")]
        elasticity: f32,
        #[serde(rename = "Translucency")]
        translucency: f32,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "Acceleration")]
        acceleration: Vector3,
        #[serde(rename = "Omega")]
        omega: Vector3,
        #[serde(rename = "DefaultScript")]
        default_script: u32,
        #[serde(rename = "DefaultScriptIntensity")]
        default_script_intensity: f32,
        #[serde(rename = "ObjectPositionSequence")]
        object_position_sequence: u16,
        #[serde(rename = "ObjectMovementSequence")]
        object_movement_sequence: u16,
        #[serde(rename = "ObjectStateSequence")]
        object_state_sequence: u16,
        #[serde(rename = "ObjectVectorSequence")]
        object_vector_sequence: u16,
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16,
        #[serde(rename = "ObjectServerControlSequence")]
        object_server_control_sequence: u16,
        #[serde(rename = "ObjectForcePositionSequence")]
        object_force_position_sequence: u16,
        #[serde(rename = "ObjectVisualDescSequence")]
        object_visual_desc_sequence: u16,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdminAccountData {
        #[serde(rename = "AccountName")]
        account_name: String,
        #[serde(rename = "BookieId")]
        bookie_id: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AdminPlayerData {
        name: String,
        #[serde(rename = "bookieId")]
        bookie_id: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VendorProfile {
        #[serde(rename = "Categories")]
        categories: ItemType,
        #[serde(rename = "MinValue")]
        min_value: u32,
        #[serde(rename = "MaxValue")]
        max_value: u32,
        #[serde(rename = "DealsMagic")]
        deals_magic: bool,
        #[serde(rename = "BuyPrice")]
        buy_price: f32,
        #[serde(rename = "SellPrice")]
        sell_price: f32,
        #[serde(rename = "CurrencyId")]
        currency_id: u32,
        #[serde(rename = "CurrencyAmount")]
        currency_amount: u32,
        #[serde(rename = "CurrencyName")]
        currency_name: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ArmorProfile {
        #[serde(rename = "ProtSlashing")]
        prot_slashing: f32,
        #[serde(rename = "ProtPiercing")]
        prot_piercing: f32,
        #[serde(rename = "ProtBludgeoning")]
        prot_bludgeoning: f32,
        #[serde(rename = "ProtCold")]
        prot_cold: f32,
        #[serde(rename = "ProtFire")]
        prot_fire: f32,
        #[serde(rename = "ProtAcid")]
        prot_acid: f32,
        #[serde(rename = "ProtNether")]
        prot_nether: f32,
        #[serde(rename = "ProtLightning")]
        prot_lightning: f32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreatureAppraisalProfile {
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Health")]
        health: u32,
        #[serde(rename = "HealthMax")]
        health_max: u32,
        #[serde(rename = "Strength")]
        strength: u32,
        #[serde(rename = "Endurance")]
        endurance: u32,
        #[serde(rename = "Quickness")]
        quickness: u32,
        #[serde(rename = "Coordination")]
        coordination: u32,
        #[serde(rename = "Focus")]
        focus: u32,
        #[serde(rename = "Self")]
        self_: u32,
        #[serde(rename = "Stamina")]
        stamina: u32,
        #[serde(rename = "Mana")]
        mana: u32,
        #[serde(rename = "StaminaMax")]
        stamina_max: u32,
        #[serde(rename = "ManaMax")]
        mana_max: u32,
        #[serde(rename = "AttrHighlight")]
        attr_highlight: AttributeMask,
        #[serde(rename = "AttrColor")]
        attr_color: AttributeMask
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WeaponProfile {
        #[serde(rename = "DamageType")]
        damage_type: DamageType,
        #[serde(rename = "Speed")]
        speed: u32,
        #[serde(rename = "Skill")]
        skill: SkillId,
        #[serde(rename = "Damage")]
        damage: u32,
        #[serde(rename = "Variance")]
        variance: f64,
        #[serde(rename = "Modifier")]
        modifier: f64,
        #[serde(rename = "Length")]
        length: f64,
        #[serde(rename = "MaxVelocity")]
        max_velocity: f64,
        #[serde(rename = "Offsense")]
        offsense: f64,
        #[serde(rename = "MaxVelocityEstimated")]
        max_velocity_estimated: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HookAppraisalProfile {
        #[serde(rename = "Flags")]
        flags: HookAppraisalFlags,
        #[serde(rename = "ValidLocations")]
        valid_locations: EquipMask,
        #[serde(rename = "AmmoType")]
        ammo_type: AmmoType
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SquelchDB {
        #[serde(rename = "AccountHash")]
        account_hash: PackableHashTable,
        #[serde(rename = "CharacterHash")]
        character_hash: PackableHashTable,
        #[serde(rename = "GlobalInfo")]
        global_info: SquelchInfo
}

// Set of information related to a squelch entry
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SquelchInfo {
        #[serde(rename = "Filters")]
        filters: PackableList,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "Account")]
        account: bool
}

// Set of information related to purchasing a housing
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HouseProfile {
        #[serde(rename = "DwellingId")]
        dwelling_id: u32,
        #[serde(rename = "OwnerId")]
        owner_id: ObjectId,
        #[serde(rename = "Flags")]
        flags: HouseBitfield,
        #[serde(rename = "MinLevel")]
        min_level: i32,
        #[serde(rename = "MaxLevel")]
        max_level: i32,
        #[serde(rename = "MinAllegRank")]
        min_alleg_rank: i32,
        #[serde(rename = "MaxAllegRank")]
        max_alleg_rank: i32,
        #[serde(rename = "MaintenanceFree")]
        maintenance_free: bool,
        #[serde(rename = "Type")]
        type_: HouseType,
        #[serde(rename = "OwnerName")]
        owner_name: String,
        #[serde(rename = "Buy")]
        buy: PackableList,
        #[serde(rename = "Rent")]
        rent: PackableList
}

// The HousePayment structure contains information about a house purchase or maintenance item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HousePayment {
        #[serde(rename = "Required")]
        required: u32,
        #[serde(rename = "Paid")]
        paid: u32,
        #[serde(rename = "WeenieClassId")]
        weenie_class_id: u32,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "PluralName")]
        plural_name: String
}

// Set of information related to owning a housing
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HouseData {
        #[serde(rename = "BuyTime")]
        buy_time: u32,
        #[serde(rename = "RentTime")]
        rent_time: u32,
        #[serde(rename = "Type")]
        type_: HouseType,
        #[serde(rename = "MaintenanceFree")]
        maintenance_free: bool,
        #[serde(rename = "Buy")]
        buy: PackableList,
        #[serde(rename = "Rent")]
        rent: PackableList,
        #[serde(rename = "Position")]
        position: Position
}

// Set of information related to house access
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HAR {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "Bitmask")]
        bitmask: u32,
        #[serde(rename = "MonarchId")]
        monarch_id: ObjectId,
        #[serde(rename = "GuestList")]
        guest_list: PackableHashTable,
        #[serde(rename = "RoommateList")]
        roommate_list: PackableList
}

// Set of information related to a house guest
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GuestInfo {
        #[serde(rename = "HasStoragePermission")]
        has_storage_permission: bool,
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Set of information related to a chess game move
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Type")]
pub enum GameMoveData {
    #[serde(rename = "0x4")]
    Type4 {
        #[serde(rename = "PlayerId")]
        player_id: ObjectId,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "IdPieceToMove")]
        id_piece_to_move: i32,
        #[serde(rename = "YGrid")]
        y_grid: i32,
    },
    #[serde(rename = "0x5")]
    Type5 {
        #[serde(rename = "PlayerId")]
        player_id: ObjectId,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "IdPieceToMove")]
        id_piece_to_move: i32,
        #[serde(rename = "YGrid")]
        y_grid: i32,
        #[serde(rename = "XTo")]
        x_to: i32,
        #[serde(rename = "YTo")]
        y_to: i32,
    },
    #[serde(rename = "0x6")]
    Type6 {
        #[serde(rename = "PlayerId")]
        player_id: ObjectId,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "IdPieceToMove")]
        id_piece_to_move: i32,
    },
}

// Set of information related to a salvage operation
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SalvageOperationsResultData {
        #[serde(rename = "SkillUsed")]
        skill_used: SkillId,
        #[serde(rename = "NotSalvagable")]
        not_salvagable: PackableList,
        #[serde(rename = "SalvageResults")]
        salvage_results: PackableList,
        #[serde(rename = "AugBonus")]
        aug_bonus: i32
}

// Set of information related to a salvage of an item
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SalvageResult {
        #[serde(rename = "Material")]
        material: MaterialType,
        #[serde(rename = "Workmanship")]
        workmanship: f64,
        #[serde(rename = "Units")]
        units: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FellowshipLockData {
        #[serde(rename = "Unknown1")]
        unknown1: u32,
        #[serde(rename = "Unknown2")]
        unknown2: u32,
        #[serde(rename = "Unknown3")]
        unknown3: u32,
        #[serde(rename = "Timestamp")]
        timestamp: u32,
        #[serde(rename = "Sequence")]
        sequence: u32
}

// Set of information for a fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship {
        #[serde(rename = "Members")]
        members: PackableHashTable,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "LeaderId")]
        leader_id: ObjectId,
        #[serde(rename = "ShareXP")]
        share_xp: bool,
        #[serde(rename = "EvenXPSplit")]
        even_xp_split: bool,
        #[serde(rename = "Open")]
        open: bool,
        #[serde(rename = "Locked")]
        locked: bool,
        #[serde(rename = "RecentlyDeparted")]
        recently_departed: PackableHashTable,
        #[serde(rename = "Locks")]
        locks: PackableHashTable
}

// The FellowInfo structure contains information about a fellowship member.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellow {
        #[serde(rename = "XPCached")]
        xp_cached: u32,
        #[serde(rename = "LumCached")]
        lum_cached: u32,
        #[serde(rename = "Level")]
        level: u32,
        #[serde(rename = "MaxHealth")]
        max_health: u32,
        #[serde(rename = "MaxStamina")]
        max_stamina: u32,
        #[serde(rename = "MaxMana")]
        max_mana: u32,
        #[serde(rename = "CurrentHealth")]
        current_health: u32,
        #[serde(rename = "CurrentStamina")]
        current_stamina: u32,
        #[serde(rename = "CurrentMana")]
        current_mana: u32,
        #[serde(rename = "ShareLoot")]
        share_loot: bool,
        #[serde(rename = "Name")]
        name: String
}

// Contains information about a contract.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContractTracker {
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "ContractId")]
        contract_id: ContractId,
        #[serde(rename = "ContractStage")]
        contract_stage: ContractStage,
        #[serde(rename = "TimeWhenDone")]
        time_when_done: i64,
        #[serde(rename = "TimeWhenRepeats")]
        time_when_repeats: i64
}

// Contains table of ContractTrackers
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContractTrackerTable {
        #[serde(rename = "ContactTrackers")]
        contact_trackers: PackableHashTable
}

// Client to Server AC packet.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct C2SPacket {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Flags")]
        flags: PacketHeaderFlags,
        #[serde(rename = "Checksum")]
        checksum: u32,
        #[serde(rename = "RecipientId")]
        recipient_id: u16,
        #[serde(rename = "TimeSinceLastPacket")]
        time_since_last_packet: u16,
        #[serde(rename = "Size")]
        size: u16,
        #[serde(rename = "Iteration")]
        iteration: u16,
        #[serde(rename = "ServerSwitch")]
        server_switch: ServerSwitchHeader,
        #[serde(rename = "RetransmitSequences")]
        retransmit_sequences: PackableList,
        #[serde(rename = "RejectSequences")]
        reject_sequences: PackableList,
        #[serde(rename = "AckSequence")]
        ack_sequence: u32,
        #[serde(rename = "LoginRequest")]
        login_request: LoginRequestHeader,
        #[serde(rename = "WorldLoginRequest")]
        world_login_request: u64,
        #[serde(rename = "ConnectResponse")]
        connect_response: u64,
        #[serde(rename = "CICMDCommand")]
        cicmd_command: CICMDCommandHeader,
        #[serde(rename = "Time")]
        time: u64,
        #[serde(rename = "EchoTime")]
        echo_time: f32,
        #[serde(rename = "Flow")]
        flow: FlowHeader,
        #[serde(rename = "Fragments")]
        fragments: BlobFragments
}

// Server to Client AC packet.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct S2CPacket {
        #[serde(rename = "Sequence")]
        sequence: u32,
        #[serde(rename = "Flags")]
        flags: PacketHeaderFlags,
        #[serde(rename = "Checksum")]
        checksum: u32,
        #[serde(rename = "RecipientId")]
        recipient_id: u16,
        #[serde(rename = "TimeSinceLastPacket")]
        time_since_last_packet: u16,
        #[serde(rename = "Size")]
        size: u16,
        #[serde(rename = "Iteration")]
        iteration: u16,
        #[serde(rename = "AckSequence")]
        ack_sequence: u32,
        #[serde(rename = "LogonServerAddr")]
        logon_server_addr: SocketAddress,
        #[serde(rename = "Referral")]
        referral: ReferralHeader,
        #[serde(rename = "ConnectRequest")]
        connect_request: ConnectRequestHeader,
        #[serde(rename = "NetError")]
        net_error: NetError,
        #[serde(rename = "NetErrorDisconnect")]
        net_error_disconnect: NetError,
        #[serde(rename = "EchoResponse")]
        echo_response: EchoResponseHeader,
        #[serde(rename = "Fragments")]
        fragments: BlobFragments
}

