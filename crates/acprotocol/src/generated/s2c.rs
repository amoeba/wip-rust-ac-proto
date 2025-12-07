use serde::{Serialize, Deserialize};

// Allegiance update cancelled
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceUpdateAborted {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Display a message in a popup message window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_PopUpString {
        #[serde(rename = "Message")]
        message: String
}

// Information describing your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_PlayerDescription {
        #[serde(rename = "BaseQualities")]
        base_qualities: ACBaseQualities,
        #[serde(rename = "Qualities")]
        qualities: ACQualities,
        #[serde(rename = "PlayerModule")]
        player_module: PlayerModule,
        #[serde(rename = "ContentProfile")]
        content_profile: PackableList,
        #[serde(rename = "InventoryPlacement")]
        inventory_placement: PackableList
}

// Returns info related to your monarch, patron and vassals.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceUpdate {
        #[serde(rename = "Rank")]
        rank: u32,
        #[serde(rename = "Profile")]
        profile: AllegianceProfile
}

// Friends list update
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_FriendsUpdate {
        #[serde(rename = "Friends")]
        friends: PackableList,
        #[serde(rename = "Type")]
        type_: FriendsUpdateType
}

// Store an item in a container.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ServerSaysContainId {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32,
        #[serde(rename = "ContainerType")]
        container_type: ContainerProperties
}

// Equip an item.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_WearItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// Titles for the current character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_CharacterTitleTable {
        #[serde(rename = "DisplayTitle")]
        display_title: u32,
        #[serde(rename = "Titles")]
        titles: PackableList
}

// Set a title for the current character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_AddOrSetCharacterTitle {
        #[serde(rename = "NewTitle")]
        new_title: u32,
        #[serde(rename = "SetAsDisplayTitle")]
        set_as_display_title: bool
}

// Close Container - Only sent when explicitly closed
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_StopViewingObjectContents {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Open the buy/sell panel for a merchant.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Vendor_VendorInfo {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: VendorProfile,
        #[serde(rename = "Items")]
        items: PackableList
}

// Opens barber UI
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_StartBarber {
        #[serde(rename = "BasePalette")]
        base_palette: DataId,
        #[serde(rename = "HeadObject")]
        head_object: DataId,
        #[serde(rename = "HeadTexture")]
        head_texture: DataId,
        #[serde(rename = "DefaultHeadTexture")]
        default_head_texture: DataId,
        #[serde(rename = "EyesTexture")]
        eyes_texture: DataId,
        #[serde(rename = "DefaultEyesTexture")]
        default_eyes_texture: DataId,
        #[serde(rename = "NoseTexture")]
        nose_texture: DataId,
        #[serde(rename = "DefaultNoseTexture")]
        default_nose_texture: DataId,
        #[serde(rename = "MouthTexture")]
        mouth_texture: DataId,
        #[serde(rename = "DefaultMouthTexture")]
        default_mouth_texture: DataId,
        #[serde(rename = "SkinPalette")]
        skin_palette: DataId,
        #[serde(rename = "HairPalette")]
        hair_palette: DataId,
        #[serde(rename = "EyesPalette")]
        eyes_palette: DataId,
        #[serde(rename = "SetupId")]
        setup_id: DataId,
        #[serde(rename = "Option1")]
        option1: i32,
        #[serde(rename = "Option2")]
        option2: i32
}

// Member left fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Quit {
        #[serde(rename = "Disband")]
        disband: bool
}

// Member dismissed from fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Dismiss {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Sent when you first open a book, contains the entire table of contents.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookOpen {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "MaxNumPages")]
        max_num_pages: u32,
        #[serde(rename = "PageData")]
        page_data: PageDataList,
        #[serde(rename = "Inscription")]
        inscription: String,
        #[serde(rename = "ScribeId")]
        scribe_id: ObjectId,
        #[serde(rename = "ScribeName")]
        scribe_name: String
}

// Response to an attempt to add a page to a book.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookAddPageResponse {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "PageNumber")]
        page_number: u32,
        #[serde(rename = "Success")]
        success: bool
}

// Response to an attempt to delete a page from a book.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookDeletePageResponse {
        #[serde(rename = "BookId")]
        book_id: ObjectId,
        #[serde(rename = "PageNumber")]
        page_number: u32,
        #[serde(rename = "Success")]
        success: bool
}

// Contains the text of a single page of a book, parchment or sign.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookPageDataResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Page")]
        page: u32,
        #[serde(rename = "PageData")]
        page_data: PageData
}

// Get Inscription Response, doesn't seem to be really used by client
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_GetInscriptionResponse {
        #[serde(rename = "Inscription")]
        inscription: String,
        #[serde(rename = "ScribeName")]
        scribe_name: String,
        #[serde(rename = "ScribeAccount")]
        scribe_account: String
}

// The result of an attempt to assess an item or creature.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_SetAppraiseInfo {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Flags")]
        flags: u32,
        #[serde(rename = "Success")]
        success: bool,
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
        #[serde(rename = "DataIdProperties")]
        data_id_properties: PackableHashTable,
        #[serde(rename = "SpellBook")]
        spell_book: PackableList,
        #[serde(rename = "ArmorProfile")]
        armor_profile: ArmorProfile,
        #[serde(rename = "CreatureProfile")]
        creature_profile: CreatureAppraisalProfile,
        #[serde(rename = "WeaponProfile")]
        weapon_profile: WeaponProfile,
        #[serde(rename = "HookProfile")]
        hook_profile: HookAppraisalProfile,
        #[serde(rename = "ArmorHighlight")]
        armor_highlight: ArmorHighlightMask,
        #[serde(rename = "ArmorColor")]
        armor_color: ArmorHighlightMask,
        #[serde(rename = "WeaponHighlight")]
        weapon_highlight: WeaponHighlightMask,
        #[serde(rename = "WeaponColor")]
        weapon_color: WeaponHighlightMask,
        #[serde(rename = "ResistHighlight")]
        resist_highlight: ResistHighlightMask,
        #[serde(rename = "ResistColor")]
        resist_color: ResistHighlightMask,
        #[serde(rename = "BaseArmorHead")]
        base_armor_head: u32,
        #[serde(rename = "BaseArmorChest")]
        base_armor_chest: u32,
        #[serde(rename = "BaseArmorGroin")]
        base_armor_groin: u32,
        #[serde(rename = "BaseArmorBicep")]
        base_armor_bicep: u32,
        #[serde(rename = "BaseArmorWrist")]
        base_armor_wrist: u32,
        #[serde(rename = "BaseArmorHand")]
        base_armor_hand: u32,
        #[serde(rename = "BaseArmorThigh")]
        base_armor_thigh: u32,
        #[serde(rename = "BaseArmorShin")]
        base_armor_shin: u32,
        #[serde(rename = "BaseArmorFoot")]
        base_armor_foot: u32
}

// ChannelBroadcast: Group Chat
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelBroadcast {
        #[serde(rename = "Channel")]
        channel: Channel,
        #[serde(rename = "Message")]
        message: String
}

// ChannelList: Provides list of characters listening to a channel, I assume in response to a command
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelList {
        #[serde(rename = "Characters")]
        characters: PackableList
}

// ChannelIndex: Provides list of channels available to the player, I assume in response to a command
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelIndex {
        #[serde(rename = "Channels")]
        channels: PackableList
}

// Set Pack Contents
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_OnViewContents {
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// ServerSaysMoveItem: Removes an item from inventory (when you place it on the ground or give it away)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ServerSaysMoveItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// HandleAttackDoneEvent: Melee attack completed
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleAttackDoneEvent {
        #[serde(rename = "Number")]
        number: u32
}

// RemoveSpell: Delete a spell from your spellbook.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// You just died.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleVictimNotificationEventSelf {
        #[serde(rename = "Message")]
        message: String
}

// Message for a death, something you killed or your own death message.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleVictimNotificationEventOther {
        #[serde(rename = "Message")]
        message: String
}

// HandleAttackerNotificationEvent: You have hit your target with a melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleAttackerNotificationEvent {
        #[serde(rename = "DefenderName")]
        defender_name: String,
        #[serde(rename = "Type")]
        type_: DamageType,
        #[serde(rename = "DamagePercent")]
        damage_percent: f32,
        #[serde(rename = "Damage")]
        damage: u32,
        #[serde(rename = "Critical")]
        critical: bool,
        #[serde(rename = "AttackConditions")]
        attack_conditions: AttackConditionsMask
}

// HandleDefenderNotificationEvent: You have been hit by a creature's melee attack.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleDefenderNotificationEvent {
        #[serde(rename = "AttackerName")]
        attacker_name: String,
        #[serde(rename = "Type")]
        type_: DamageType,
        #[serde(rename = "DamagePercent")]
        damage_percent: f32,
        #[serde(rename = "Damage")]
        damage: u32,
        #[serde(rename = "Location")]
        location: DamageLocation,
        #[serde(rename = "Critical")]
        critical: bool,
        #[serde(rename = "AttackConditions")]
        attack_conditions: AttackConditionsMask
}

// HandleEvasionAttackerNotificationEvent: Your target has evaded your melee attack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleEvasionAttackerNotificationEvent {
        #[serde(rename = "DefenderName")]
        defender_name: String
}

// HandleEvasionDefenderNotificationEvent: You have evaded a creature's melee attack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleEvasionDefenderNotificationEvent {
        #[serde(rename = "AttackerName")]
        attacker_name: String
}

// HandleCommenceAttackEvent: Start melee attack
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandleCommenceAttackEvent {}

// QueryHealthResponse: Update a creature's health bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_QueryHealthResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Health")]
        health: f32
}

// QueryAgeResponse: happens when you do /age in the game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_QueryAgeResponse {
        #[serde(rename = "TargetName")]
        target_name: String,
        #[serde(rename = "Age")]
        age: String
}

// UseDone: Ready. Previous action complete
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_UseDone {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Allegiance update finished
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceUpdateDone {
        #[serde(rename = "FailureType")]
        failure_type: WeenieError
}

// Fellow update is done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_FellowUpdateDone {}

// Fellow stats are done
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_FellowStatsDone {}

// Close Assess Panel
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_AppraiseDone {
        #[serde(rename = "Unknown")]
        unknown: u32
}

// Ping Reply
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_ReturnPing {}

// Squelch and Filter List
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_SetSquelchDB {
        #[serde(rename = "SquelchDB")]
        squelch_db: SquelchDB
}

// RegisterTrade: Send to begin a trade and display the trade window
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_RegisterTrade {
        #[serde(rename = "InitiatorId")]
        initiator_id: ObjectId,
        #[serde(rename = "PartnerId")]
        partner_id: ObjectId,
        #[serde(rename = "Stamp")]
        stamp: i64
}

// OpenTrade: Open trade window
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_OpenTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// CloseTrade: End trading
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_CloseTrade {
        #[serde(rename = "Reason")]
        reason: EndTradeReason
}

// RemoveFromTrade: Item was removed from trade window
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_AddToTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Side")]
        side: TradeSide
}

// Removes an item from the trade window, not sure if this is used still?
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_RemoveFromTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Side")]
        side: TradeSide
}

// AcceptTrade: The trade was accepted
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_AcceptTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// DeclineTrade: The trade was declined
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_DeclineTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// ResetTrade: The trade window was reset
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_ResetTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// TradeFailure: Failure to add a trade item
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Trade_TradeFailure {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Reason")]
        reason: u32
}

// ClearTradeAcceptance: Failure to complete a trade
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade_ClearTradeAcceptance {}

// Buy a dwelling or pay maintenance
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseProfile {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: HouseProfile
}

// House panel information for owners.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseData {
        #[serde(rename = "Data")]
        data: HouseData
}

// House Data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseStatus {
        #[serde(rename = "NoticeType")]
        notice_type: u32
}

// Update Rent Time
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateRentTime {
        #[serde(rename = "RentTime")]
        rent_time: u32
}

// Update Rent Payment
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateRentPayment {
        #[serde(rename = "Rent")]
        rent: PackableList
}

// Update Restrictions
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateRestrictions {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "Restrictions")]
        restrictions: RestrictionDB
}

// House Guest List
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_UpdateHAR {
        #[serde(rename = "GuestList")]
        guest_list: HAR
}

// House Profile
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_HouseTransaction {
        #[serde(rename = "NoticeType")]
        notice_type: u32
}

// Update an item's mana bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item_QueryItemManaResponse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Mana")]
        mana: f32,
        #[serde(rename = "Success")]
        success: bool
}

// Display a list of available dwellings in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct House_AvailableHouses {
        #[serde(rename = "Type")]
        type_: HouseType,
        #[serde(rename = "Houses")]
        houses: PackableList,
        #[serde(rename = "NumHouses")]
        num_houses: i32
}

// Display a confirmation panel.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ConfirmationRequest {
        #[serde(rename = "ConfirmationType")]
        confirmation_type: ConfirmationType,
        #[serde(rename = "ContextId")]
        context_id: u32,
        #[serde(rename = "Text")]
        text: String
}

// Confirmation done
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ConfirmationDone {
        #[serde(rename = "ConfirmationType")]
        confirmation_type: ConfirmationType,
        #[serde(rename = "ContextId")]
        context_id: u32
}

// Display an allegiance login/logout message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceLoginNotificationEvent {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "IsLoggedIn")]
        is_logged_in: bool
}

// Returns data for a player's allegiance information
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceInfoResponseEvent {
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "Profile")]
        profile: AllegianceProfile
}

// Joining game response
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_JoinGameResponse {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32
}

// Start game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_StartGame {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32
}

// Move response
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_MoveResponse {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "MoveResult")]
        move_result: ChessMoveResult
}

// Opponent Turn
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_OpponentTurn {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "GameMove")]
        game_move: GameMoveData
}

// Opponent Stalemate State
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_OpponentStalemateState {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: i32,
        #[serde(rename = "On")]
        on: bool
}

// Display a status message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_WeenieError {
        #[serde(rename = "Type")]
        type_: WeenieError
}

// Display a parameterized status message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_WeenieErrorWithString {
        #[serde(rename = "Type")]
        type_: WeenieErrorWithString,
        #[serde(rename = "Text")]
        text: String
}

// End of Chess game
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Game_GameOver {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "TeamWinner")]
        team_winner: i32
}

// Set Turbine Chat channel numbers.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChatRoomTracker {
        #[serde(rename = "AllegianceRoomId")]
        allegiance_room_id: u32,
        #[serde(rename = "GeneralChatRoomId")]
        general_chat_room_id: u32,
        #[serde(rename = "TradeChatRoomId")]
        trade_chat_room_id: u32,
        #[serde(rename = "LFGChatRoomId")]
        lfg_chat_room_id: u32,
        #[serde(rename = "RoleplayChatRoomId")]
        roleplay_chat_room_id: u32,
        #[serde(rename = "OlthoiChatRoomId")]
        olthoi_chat_room_id: u32,
        #[serde(rename = "SocietyChatRoomId")]
        society_chat_room_id: u32,
        #[serde(rename = "SocietyCelestialHandChatRoomId")]
        society_celestial_hand_chat_room_id: u32,
        #[serde(rename = "SocietyEldrichWebChatRoomId")]
        society_eldrich_web_chat_room_id: u32,
        #[serde(rename = "SocietyRadiantBloodChatRoomId")]
        society_radiant_blood_chat_room_id: u32
}

// TODO: QueryPluginList
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginList {}

// TODO: QueryPlugin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPlugin {}

// TODO: QueryPluginResponse
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginResponse2 {}

// Salvage operation results
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_SalvageOperationsResultData {
        #[serde(rename = "Result")]
        result: SalvageOperationsResultData
}

// Someone has sent you a @tell.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearDirectSpeech {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "Type")]
        type_: ChatFragmentType,
        #[serde(rename = "SecretFlags")]
        secret_flags: u32
}

// Create or join a fellowship
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_FullUpdate {
        #[serde(rename = "Fellowship")]
        fellowship: Fellowship
}

// Disband your fellowship.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Disband {}

// Add/Update a member to your fellowship.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_UpdateFellow {
        #[serde(rename = "Fellow")]
        fellow: Fellow,
        #[serde(rename = "UpdateType")]
        update_type: FellowUpdateType
}

// Add a spell to your spellbook.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_UpdateSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Apply an enchantment to your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_UpdateEnchantment {
        #[serde(rename = "Enchantment")]
        enchantment: Enchantment
}

// Remove an enchantment from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveEnchantment {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Update multiple enchantments from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_UpdateMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// Remove multiple enchantments from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// Silently remove all enchantments from your character, e.g. when you die (no message in the chat window).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Magic_PurgeEnchantments {}

// Silently remove An enchantment from your character.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_DispelEnchantment {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Silently remove multiple enchantments from your character (no message in the chat window).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Magic_DispelMultipleEnchantments {
        #[serde(rename = "Enchantments")]
        enchantments: PackableList
}

// A portal storm is brewing.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStormBrewing {
        #[serde(rename = "Extent")]
        extent: f32
}

// A portal storm is imminent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStormImminent {
        #[serde(rename = "Extent")]
        extent: f32
}

// You have been portal stormed.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStorm {}

// The portal storm has subsided.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Misc_PortalStormSubsided {}

// Display a status message on the Action Viewscreen (the red text overlaid on the 3D area).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_TransientString {
        #[serde(rename = "Message")]
        message: String
}

// Remove all bad enchantments from your character.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Magic_PurgeBadEnchantments {}

// Sends all contract data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_SendClientContractTrackerTable {
        #[serde(rename = "ContractTracker")]
        contract_tracker: ContractTrackerTable
}

// Updates a contract data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Social_SendClientContractTracker {
        #[serde(rename = "ContractTracker")]
        contract_tracker: ContractTracker,
        #[serde(rename = "DeleteContract")]
        delete_contract: bool,
        #[serde(rename = "SetAsDisplayContract")]
        set_as_display_contract: bool
}

// Sent every time an object you are aware of ceases to exist. Merely running out of range does not generate this message - in that case, the client just automatically stops tracking it after receiving no updates for a while (which I presume is a very short while).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ServerSaysRemove {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Failure to give an item
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_ServerSaysAttemptFailed {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Reason")]
        reason: WeenieError
}

// For stackable items, this changes the number of items in the stack.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_UpdateStackSize {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32,
        #[serde(rename = "NewValue")]
        new_value: u32
}

// A Player Kill occurred nearby (also sent for suicides).
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Combat_HandlePlayerDeathEvent {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "KilledId")]
        killed_id: ObjectId,
        #[serde(rename = "KillerId")]
        killer_id: ObjectId
}

// Remove an int property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveIntEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInt
}

// Remove an int property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveIntEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInt
}

// Remove a bool property from the charactert
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveBoolEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyBool
}

// Remove a bool property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveBoolEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyBool
}

// Remove a float property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveFloatEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyFloat
}

// Remove a float property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveFloatEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyFloat
}

// Remove a string property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveStringEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyString
}

// Remove a string property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveStringEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyString
}

// Remove an dataId property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveDataIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyDataId
}

// Remove an dataId property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveDataIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyDataId
}

// Remove an instanceId property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveInstanceIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInstanceId
}

// Remove an instanceId property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveInstanceIdEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInstanceId
}

// Remove a position property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemovePositionEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyPosition
}

// Remove a position property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemovePositionEvent {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyPosition
}

// Remove an int64 property from the character
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateRemoveInt64Event {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Type")]
        type_: PropertyInt64
}

// Remove an int64 property from an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_RemoveInt64Event {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Type")]
        type_: PropertyInt64
}

// Set or update a Character Int property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateInt {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInt,
        #[serde(rename = "Value")]
        value: i32
}

// Set or update an Object Int property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateInt {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyInt,
        #[serde(rename = "Value")]
        value: i32
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateInt64 {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInt64,
        #[serde(rename = "Value")]
        value: i64
}

// Set or update a Character Int64 property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateInt64 {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyInt64,
        #[serde(rename = "Value")]
        value: i64
}

// Set or update a Character Boolean property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateBool {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyBool,
        #[serde(rename = "Value")]
        value: bool
}

// Set or update an Object Boolean property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateBool {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyBool,
        #[serde(rename = "Value")]
        value: bool
}

// Set or update an Object float property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateFloat {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyFloat,
        #[serde(rename = "Value")]
        value: f32
}

// Set or update an Object float property value
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateFloat {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyFloat,
        #[serde(rename = "Value")]
        value: f32
}

// Set or update an Object String property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateString {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyString,
        #[serde(rename = "Value")]
        value: String
}

// Set or update an Object String property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateString {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyString,
        #[serde(rename = "Value")]
        value: String
}

// Set or update an Object DId property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateDataId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyDataId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update an Object DId property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateDataId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyDataId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a IId property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateInstanceId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyInstanceId,
        #[serde(rename = "Value")]
        value: ObjectId
}

// Set or update an Object IId property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateInstanceId {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyInstanceId,
        #[serde(rename = "Value")]
        value: ObjectId
}

// Set or update a Character Position property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdatePosition {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: PropertyPosition,
        #[serde(rename = "Value")]
        value: Position
}

// Set or update a Character Position property value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdatePosition {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: PropertyPosition,
        #[serde(rename = "Value")]
        value: Position
}

// Set or update a Character Skill value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateSkill {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: Skill
}

// Set or update a Character Skill value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateSkill {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: Skill
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateSkillLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Skill Level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateSkillLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Skill state
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateSkillAC {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: SkillAdvancementClass
}

// Set or update a Character Skill state
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateSkillAC {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: SkillId,
        #[serde(rename = "Value")]
        value: SkillAdvancementClass
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttribute {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: AttributeInfo
}

// Set or update a Character Attribute value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttribute {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: AttributeInfo
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttributeLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Attribute Level
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttributeLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: AttributeId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttribute2nd {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: VitalId,
        #[serde(rename = "Value")]
        value: SecondaryAttributeInfo
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttribute2nd {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: VitalId,
        #[serde(rename = "Value")]
        value: SecondaryAttributeInfo
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_PrivateUpdateAttribute2ndLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "Key")]
        key: CurVitalId,
        #[serde(rename = "Value")]
        value: u32
}

// Set or update a Character Vital value
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Qualities_UpdateAttribute2ndLevel {
        #[serde(rename = "Sequence")]
        sequence: u8,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Key")]
        key: CurVitalId,
        #[serde(rename = "Value")]
        value: u32
}

// Indirect '/e' text.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearEmote {
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Text")]
        text: String
}

// Contains the text associated with an emote action.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearSoulEmote {
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Text")]
        text: String
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearSpeech {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// A message to be displayed in the chat window, spoken by a nearby player, NPC or creature
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_HearRangedSpeech {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "SenderId")]
        sender_id: ObjectId,
        #[serde(rename = "Range")]
        range: f32,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// This appears to be an admin command to change the environment (light, fog, sounds, colors)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_Environs {
        #[serde(rename = "EnvrionOption")]
        envrion_option: EnvrionChangeType
}

// Sets both the position and movement, such as when materializing at a lifestone
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_PositionAndMovementEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Position")]
        position: PositionPack,
        #[serde(rename = "MovementData")]
        movement_data: MovementData
}

// Sent whenever a character changes their clothes. It contains the entire description of what their wearing (and possibly their facial features as well). This message is only sent for changes, when the character is first created, the body of this message is included inside the creation message.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ObjDescEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectDescription")]
        object_description: ObjDesc,
        #[serde(rename = "InstanceSequence")]
        instance_sequence: u16,
        #[serde(rename = "VisualDescSequence")]
        visual_desc_sequence: u16
}

// Sets the player visual desc, TODO confirm this
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_SetPlayerVisualDesc {
        #[serde(rename = "ObjectDescription")]
        object_description: ObjDesc
}

// Character creation screen initilised.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "ResponseType")]
pub enum Character_CharGenVerificationResponse {
    #[serde(rename = "0x1")]
    Type1 {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "SecondsUntilDeletion")]
        seconds_until_deletion: u32,
    },
}

// Sent when your subsciption is about to expire
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_AwaitingSubscriptionExpiration {
        #[serde(rename = "SecondsRemaining")]
        seconds_remaining: u32
}

// Instructs the client to return to 2D mode - the character list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Login_LogOffCharacter {}

// A character was marked for deletetion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterDelete {}

// The list of characters on the current account.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_LoginCharacterSet {
        #[serde(rename = "Status")]
        status: u32,
        #[serde(rename = "Characters")]
        characters: PackableList,
        #[serde(rename = "DeletedCharacters")]
        deleted_characters: PackableList,
        #[serde(rename = "NumAllowedCharacters")]
        num_allowed_characters: u32,
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "UseTurbineChat")]
        use_turbine_chat: bool,
        #[serde(rename = "HasThroneofDestiny")]
        has_throneof_destiny: bool
}

// Failure to log in
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterError {
        #[serde(rename = "Reason")]
        reason: CharacterErrorType
}

// Create an object somewhere in the world
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_CreateObject {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectDescription")]
        object_description: ObjDesc,
        #[serde(rename = "PhysicsDescription")]
        physics_description: PhysicsDesc,
        #[serde(rename = "WeenieDescription")]
        weenie_description: PublicWeenieDesc
}

// Login of player
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_CreatePlayer {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId
}

// Sent whenever an object is being deleted from the scene.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_DeleteObject {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16
}

// Sets the position/motion of an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_PositionEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Position")]
        position: PositionPack
}

// Sets the parent for an object, eg. equipting an object.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_ParentEvent {
        #[serde(rename = "ParentId")]
        parent_id: ObjectId,
        #[serde(rename = "ChildId")]
        child_id: ObjectId,
        #[serde(rename = "Location")]
        location: ParentLocation,
        #[serde(rename = "Placement")]
        placement: Placement,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ChildPositionSequence")]
        child_position_sequence: u16
}

// Sent when picking up an object
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory_PickupEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectPositionSequence")]
        object_position_sequence: u16
}

// Set's the current state of the object. Client appears to only process the following state changes post creation: NoDraw, LightingOn, Hidden
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_SetState {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "NewState")]
        new_state: PhysicsState,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectStateSequence")]
        object_state_sequence: u16
}

// These are animations. Whenever a human, monster or object moves - one of these little messages is sent. Even idle emotes (like head scratching and nodding) are sent in this manner.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_SetObjectMovement {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "MovementData")]
        movement_data: MovementData
}

// Changes an objects vector, for things like jumping
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movement_VectorUpdate {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Velocity")]
        velocity: Vector3,
        #[serde(rename = "Omega")]
        omega: Vector3,
        #[serde(rename = "ObjectInstanceSequence")]
        object_instance_sequence: u16,
        #[serde(rename = "ObjectVectorSequence")]
        object_vector_sequence: u16
}

// Applies a sound effect.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Effects_SoundEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SoundType")]
        sound_type: Sound,
        #[serde(rename = "Volume")]
        volume: f32
}

// Instructs the client to show the portal graphic.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Effects_PlayerTeleport {
        #[serde(rename = "ObjectTeleportSequence")]
        object_teleport_sequence: u16
}

// Instructs the client to play a script. (Not seen so far, maybe prefered PlayScriptType)
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Effects_PlayScriptId {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ScriptId")]
        script_id: DataId
}

// Applies an effect with visual and sound by providing the type to be looked up in the Physics Script Table
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Effects_PlayScriptType {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ScriptType")]
        script_type: i32,
        #[serde(rename = "Speed")]
        speed: f32
}

// Account has been banned
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_AccountBanned {
        #[serde(rename = "BannedUntil")]
        banned_until: u32,
        #[serde(rename = "Text")]
        text: String
}

// Admin Receive Account Data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_ReceiveAccountData {
        #[serde(rename = "Unknown")]
        unknown: u32,
        #[serde(rename = "AdminAccountData")]
        admin_account_data: PackableList
}

// Admin Receive Player Data
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Admin_ReceivePlayerData {
        #[serde(rename = "Unknown")]
        unknown: i32,
        #[serde(rename = "AdminPlayerData")]
        admin_player_data: PackableList
}

// Update an existing object's data.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Item_UpdateObject {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ObjectDesc")]
        object_desc: ObjDesc,
        #[serde(rename = "PhysicsDesc")]
        physics_desc: PhysicsDesc,
        #[serde(rename = "WeenieDesc")]
        weenie_desc: PublicWeenieDesc
}

// Account has been booted
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_AccountBooted {
        #[serde(rename = "AdditionalReasonText")]
        additional_reason_text: String,
        #[serde(rename = "ReasonText")]
        reason_text: String
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "BlobDispatchType")]
pub enum Communication_TurbineChat {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x02")]
    Type01 {
        #[serde(rename = "MessageSize")]
        message_size: u32,
        #[serde(rename = "Type")]
        type_: TurbineChatType,
        #[serde(rename = "TargetType")]
        target_type: i32,
        #[serde(rename = "TargetId")]
        target_id: i32,
        #[serde(rename = "TransportType")]
        transport_type: i32,
        #[serde(rename = "TransportId")]
        transport_id: i32,
        #[serde(rename = "Cookie")]
        cookie: i32,
        #[serde(rename = "PayloadSize")]
        payload_size: u32,
        #[serde(rename = "ContextId")]
        context_id: u32,
        #[serde(rename = "ResponseId")]
        response_id: u32,
        #[serde(rename = "MethodId")]
        method_id: u32,
        #[serde(rename = "HResult")]
        h_result: i32,
    },
}

// Switch from the character display to the game display.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Login_EnterGame_ServerReady {}

// Display a message in the chat window.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Communication_TextboxString {
        #[serde(rename = "Text")]
        text: String,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// The name of the current world.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Login_WorldInfo {
        #[serde(rename = "Connections")]
        connections: u32,
        #[serde(rename = "MaxConnections")]
        max_connections: u32,
        #[serde(rename = "WorldName")]
        world_name: String
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "Compression")]
pub enum DDD_DataMessage {
    #[serde(rename = "0x01")]
    Type01 {
        #[serde(rename = "DatFile")]
        dat_file: DatFileType,
        #[serde(rename = "ResourceType")]
        resource_type: u32,
        #[serde(rename = "ResourceId")]
        resource_id: DataId,
        #[serde(rename = "Iteration")]
        iteration: u32,
        #[serde(rename = "Version")]
        version: u32,
        #[serde(rename = "DataSize")]
        data_size: u32,
        #[serde(rename = "FileSize")]
        file_size: u32,
    },
}

// DDD error
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_ErrorMessage {
        #[serde(rename = "ResourceType")]
        resource_type: u32,
        #[serde(rename = "ResourceId")]
        resource_id: DataId,
        #[serde(rename = "RError")]
        r_error: u32
}

// A list of dat files that need to be patched
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_BeginDDDMessage {
        #[serde(rename = "DataExpected")]
        data_expected: u32,
        #[serde(rename = "Revisions")]
        revisions: PackableList
}

// Add or update a dat file Resource.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct DDD_InterrogationMessage {
        #[serde(rename = "ServersRegion")]
        servers_region: u32,
        #[serde(rename = "NameRuleLanguage")]
        name_rule_language: u32,
        #[serde(rename = "ProductId")]
        product_id: u32,
        #[serde(rename = "SupportedLanguages")]
        supported_languages: PackableList
}

// Ends DDD update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDD_OnEndDDD {}

