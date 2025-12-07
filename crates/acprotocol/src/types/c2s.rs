use serde::{Serialize, Deserialize};

// Set a single character option.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_PlayerOptionChangedEvent {
        #[serde(rename = "Option")]
        option: CharacterOptions1,
        #[serde(rename = "Value")]
        value: bool
}

// Starts a melee attack against a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_TargetedMeleeAttack {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Height")]
        height: AttackHeight,
        #[serde(rename = "Power")]
        power: f32
}

// Starts a missle attack against a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_TargetedMissileAttack {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Height")]
        height: AttackHeight,
        #[serde(rename = "Accuracy")]
        accuracy: f32
}

// Set AFK mode.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_SetAFKMode {
        #[serde(rename = "AFK")]
        afk: bool
}

// Set AFK message.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_SetAFKMessage {
        #[serde(rename = "Message")]
        message: String
}

// Talking
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_Talk {
        #[serde(rename = "Message")]
        message: String
}

// Removes a friend
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Social_RemoveFriend {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Adds a friend
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Social_AddFriend {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Store an item in a container.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_PutItemInContainer {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32
}

// Gets and weilds an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_GetAndWieldItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask
}

// Drop an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_DropItem {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Swear allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SwearAllegiance {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Break allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_BreakAllegiance {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Allegiance update request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_UpdateRequest {
        #[serde(rename = "On")]
        on: bool
}

// Clears friend list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Social_ClearFriends {}

// Teleport to the PKLite Arena
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToPKLArena {}

// Teleport to the PK Arena
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToPKArena {}

// Sets a character's display title
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Social_SetDisplayCharacterTitle {
        #[serde(rename = "TitleId")]
        title_id: u32
}

// Query the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_QueryAllegianceName {}

// Clears the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearAllegianceName {}

// Direct message by Id
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_TalkDirect {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "TargetId")]
        target_id: ObjectId
}

// Sets the allegiance name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceName {
        #[serde(rename = "Name")]
        name: String
}

// Attempt to use an item with a target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_UseWithTargetEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "TargetId")]
        target_id: ObjectId
}

// Attempt to use an item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_UseEvent {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Sets an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceOfficer {
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "Level")]
        level: AllegianceOfficerLevel
}

// Sets an allegiance officer title for a given level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceOfficerTitle {
        #[serde(rename = "Level")]
        level: AllegianceOfficerLevel,
        #[serde(rename = "Title")]
        title: String
}

// List the allegiance officer titles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ListAllegianceOfficerTitles {}

// Clears the allegiance officer titles
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearAllegianceOfficerTitles {}

// Perform the allegiance lock action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_DoAllegianceLockAction {
        #[serde(rename = "Action")]
        action: AllegianceLockAction
}

// Sets a person as an approved vassal
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetAllegianceApprovedVassal {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Gags a person in allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceChatGag {
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "On")]
        on: bool
}

// Perform the allegiance house action
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_DoAllegianceHouseAction {
        #[serde(rename = "Action")]
        action: AllegianceHouseAction
}

// Spend XP to raise a vital.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainAttribute2nd {
        #[serde(rename = "Type")]
        type_: VitalId,
        #[serde(rename = "Experience")]
        experience: u32
}

// Spend XP to raise an attribute.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainAttribute {
        #[serde(rename = "Type")]
        type_: AttributeId,
        #[serde(rename = "Experience")]
        experience: u32
}

// Spend XP to raise a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainSkill {
        #[serde(rename = "Skill")]
        skill: SkillId,
        #[serde(rename = "Experience")]
        experience: u32
}

// Spend skill credits to train a skill.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Train_TrainSkillAdvancementClass {
        #[serde(rename = "Skill")]
        skill: SkillId,
        #[serde(rename = "Credits")]
        credits: u32
}

// Cast a spell with no target.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Magic_CastUntargetedSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Cast a spell on a target
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Magic_CastTargetedSpell {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Changes the combat mode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_ChangeCombatMode {
        #[serde(rename = "Mode")]
        mode: CombatMode
}

// Merges one stack with another
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableMerge {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32
}

// Split a stack and place it into a container
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableSplitToContainer {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "ContainerId")]
        container_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32,
        #[serde(rename = "Amount")]
        amount: u32
}

// Split a stack and place it into the world
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableSplitTo3D {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32
}

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_ModifyCharacterSquelch {
        #[serde(rename = "Add")]
        add: bool,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// Changes an account squelch
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_ModifyAccountSquelch {
        #[serde(rename = "Add")]
        add: bool,
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Changes the global filters, /filter -type as well as /chat and /notell
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_ModifyGlobalSquelch {
        #[serde(rename = "Add")]
        add: bool,
        #[serde(rename = "Type")]
        type_: ChatFragmentType
}

// Direct message by name
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_TalkDirectByName {
        #[serde(rename = "Message")]
        message: String,
        #[serde(rename = "TargetName")]
        target_name: String
}

// Buy from a vendor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vendor_Buy {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList,
        #[serde(rename = "AlternateCurrencyId")]
        alternate_currency_id: u32
}

// Sell to a vendor
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vendor_Sell {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Teleport to your lifestone. (/lifestone)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToLifestone {}

// The client is ready for the character to materialize after portalling or logging on.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_LoginCompleteNotification {}

// Create a fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Create {
        #[serde(rename = "Name")]
        name: String,
        #[serde(rename = "ShareXP")]
        share_xp: bool
}

// Quit the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Quit {
        #[serde(rename = "Disband")]
        disband: bool
}

// Dismiss a player from the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Dismiss {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Recruit a player to the fellowship
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_Recruit {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Update request
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_UpdateRequest {
        #[serde(rename = "On")]
        on: bool
}

// Request update to book data (seems to be sent after failed add page)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookAddPage {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Updates a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookModifyPage {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "PageNum")]
        page_num: i32,
        #[serde(rename = "PageText")]
        page_text: String
}

// Add a page to a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookData {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Removes a page from a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookDeletePage {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "PageNum")]
        page_num: i32
}

// Requests data for a page in a book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Writing_BookPageData {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "PageNum")]
        page_num: i32
}

// Sets the inscription on an object
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Writing_SetInscription {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Inscription")]
        inscription: String
}

// Appraise something
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item_Appraise {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Give an item to someone.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_GiveObjectRequest {
        #[serde(rename = "TargetId")]
        target_id: ObjectId,
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Amount")]
        amount: u32
}

// Advocate Teleport
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Advocate_Teleport {
        #[serde(rename = "ObjectId")]
        object_id: String,
        #[serde(rename = "Destination")]
        destination: Position
}

// Sends an abuse report.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_AbuseLogRequest {
        #[serde(rename = "Character")]
        character: String,
        #[serde(rename = "Status")]
        status: u32,
        #[serde(rename = "Complaint")]
        complaint: String
}

// Joins a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_AddToChannel {
        #[serde(rename = "Channel")]
        channel: Channel
}

// Leaves a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_RemoveFromChannel {
        #[serde(rename = "Channel")]
        channel: Channel
}

// Sends a message to a chat channel
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelBroadcast {
        #[serde(rename = "Channel")]
        channel: Channel,
        #[serde(rename = "SenderName")]
        sender_name: String,
        #[serde(rename = "Message")]
        message: String
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelList {
        #[serde(rename = "Channel")]
        channel: Channel
}

// Requests a channel index
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_ChannelIndex {}

// Stop viewing the contents of a container
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_NoLongerViewingContents {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Splits an item to a wield location.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_StackableSplitToWield {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Slot")]
        slot: EquipMask,
        #[serde(rename = "Amount")]
        amount: i32
}

// Add an item to the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_AddShortCut {
        #[serde(rename = "Shortcut")]
        shortcut: ShortCutData
}

// Remove an item from the shortcut bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_RemoveShortCut {
        #[serde(rename = "Index")]
        index: u32
}

// Set multiple character options.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterOptionsEvent {
        #[serde(rename = "Options")]
        options: PlayerModule
}

// Removes a spell from the spell book
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Magic_RemoveSpell {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId
}

// Cancels an attack
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_CancelAttack {}

// Query's a creatures health
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Combat_QueryHealth {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Query a character's age
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_QueryAge {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Query a character's birth day
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_QueryBirth {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Emote message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_Emote {
        #[serde(rename = "Message")]
        message: String
}

// Soul emote message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Communication_SoulEmote {
        #[serde(rename = "Message")]
        message: String
}

// Add a spell to a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_AddSpellFavorite {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId,
        #[serde(rename = "Index")]
        index: u32,
        #[serde(rename = "SpellBar")]
        spell_bar: u32
}

// Remove a spell from a spell bar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_RemoveSpellFavorite {
        #[serde(rename = "SpellId")]
        spell_id: LayeredSpellId,
        #[serde(rename = "SpellBar")]
        spell_bar: u32
}

// Request a ping
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_RequestPing {}

// Starts trading with another player.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade_OpenTradeNegotiations {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Ends trading, when trade window is closed?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade_CloseTradeNegotiations {}

// Adds an object to the trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade_AddToTrade {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "SlotIndex")]
        slot_index: u32
}

// Accepts a trade.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade_AcceptTrade {
        #[serde(rename = "Contents")]
        contents: Trade
}

// Declines a trade, when cancel is clicked?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade_DeclineTrade {}

// Resets the trade, when clear all is clicked?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Trade_ResetTrade {}

// Clears the player's corpse looting consent list, /consent clear
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_ClearPlayerConsentList {}

// Display the player's corpse looting consent list, /consent who 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_DisplayPlayerConsentList {}

// Remove your corpse looting permission for the given player, /consent remove 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_RemoveFromPlayerConsentList {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Grants a player corpse looting permission, /permit add
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_AddPlayerPermission {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Buy a house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_BuyHouse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Query your house info, during signin
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_QueryHouse {}

// Abandons your house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_AbandonHouse {}

// Revokes a player's corpse looting permission, /permit remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_RemovePlayerPermission {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Pay rent for a house
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_RentHouse {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Sets a new fill complevel for a component
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_SetDesiredComponentLevel {
        #[serde(rename = "Wcid")]
        wcid: u32,
        #[serde(rename = "Amount")]
        amount: u32
}

// Adds a guest to your house guest list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_AddPermanentGuest {
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Removes a specific player from your house guest list, /house guest remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_RemovePermanentGuest {
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Sets your house open status
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_SetOpenHouseStatus {
        #[serde(rename = "OpenHouse")]
        open_house: bool
}

// Changes a specific players storage permission, /house storage add/remove
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_ChangeStoragePermission {
        #[serde(rename = "GuestName")]
        guest_name: String,
        #[serde(rename = "HasPermission")]
        has_permission: bool
}

// Boots a specific player from your house /house boot
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_BootSpecificHouseGuest {
        #[serde(rename = "GuestName")]
        guest_name: String
}

// Removes all storage permissions, /house storage remove_all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_RemoveAllStoragePermission {}

// Requests your full guest list, /house guest list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_RequestFullGuestList {}

// Sets the allegiance message of the day, /allegiance motd set
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_SetMotd {
        #[serde(rename = "Message")]
        message: String
}

// Query the motd, /allegiance motd
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_QueryMotd {}

// Clear the motd, /allegiance motd clear
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearMotd {}

// Gets SlumLord info, sent after getting a failed house transaction
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_QueryLord {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Adds all to your storage permissions, /house storage add -all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_AddAllStoragePermission {}

// Removes all guests, /house guest remove_all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_RemoveAllPermanentGuests {}

// Boot everyone from your house, /house boot -all
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_BootEveryone {}

// Teleports you to your house, /house recall
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_TeleToHouse {}

// Queries an item's mana
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Item_QueryItemMana {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Modify whether house hooks are visibile or not, /house hooks on/off
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_SetHooksVisibility {
        #[serde(rename = "Visible")]
        visible: bool
}

// Modify whether allegiance members are guests, /house guest add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_ModifyAllegianceGuestPermission {
        #[serde(rename = "Add")]
        add: bool
}

// Modify whether allegiance members can access storage, /house storage add_allegiance/remove_allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_ModifyAllegianceStoragePermission {
        #[serde(rename = "Add")]
        add: bool
}

// Joins a chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game_Join {
        #[serde(rename = "GameId")]
        game_id: u32,
        #[serde(rename = "Team")]
        team: u32
}

// Quits a chess game
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game_Quit {}

// Makes a chess move
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game_Move {
        #[serde(rename = "XFrom")]
        x_from: i32,
        #[serde(rename = "YFrom")]
        y_from: i32,
        #[serde(rename = "XTo")]
        x_to: i32,
        #[serde(rename = "YTo")]
        y_to: i32
}

// Skip a move?
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game_MovePass {}

// Offer or confirm stalemate
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Game_Stalemate {
        #[serde(rename = "On")]
        on: bool
}

// Lists available house /house available
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_ListAvailableHouses {
        #[serde(rename = "Type")]
        type_: HouseType
}

// Confirms a dialog
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_ConfirmationResponse {
        #[serde(rename = "Type")]
        type_: ConfirmationType,
        #[serde(rename = "Context")]
        context: u32,
        #[serde(rename = "Accepted")]
        accepted: bool
}

// Boots a player from the allegiance, optionally all characters on their account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_BreakAllegianceBoot {
        #[serde(rename = "BooteeName")]
        bootee_name: String,
        #[serde(rename = "AccountBoot")]
        account_boot: bool
}

// Teleports player to their allegiance housing, /house mansion_recall
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct House_TeleToMansion {}

// Player is commiting suicide
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_Suicide {}

// Request allegiance info for a player
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceInfoRequest {
        #[serde(rename = "TargetName")]
        target_name: String
}

// Salvages items
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Inventory_CreateTinkeringTool {
        #[serde(rename = "ToolId")]
        tool_id: ObjectId,
        #[serde(rename = "Items")]
        items: PackableList
}

// Changes the spell book filter
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_SpellbookFilterEvent {
        #[serde(rename = "Options")]
        options: SpellBookFilterOptions
}

// Teleport to the marketplace
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_TeleToMarketplace {}

// Enter PKLite mode
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_EnterPKLite {}

// Fellowship Assign a new leader
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_AssignNewLeader {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// Fellowship Change openness
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fellowship_ChangeFellowOpeness {
        #[serde(rename = "Open")]
        open: bool
}

// Boots a player from the allegiance chat
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AllegianceChatBoot {
        #[serde(rename = "CharacterName")]
        character_name: String,
        #[serde(rename = "Reason")]
        reason: String
}

// Bans a player from the allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_AddAllegianceBan {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Removes a player ban from the allegiance
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_RemoveAllegianceBan {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// Display allegiance bans
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ListAllegianceBans {}

// Removes an allegiance officer
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_RemoveAllegianceOfficer {
        #[serde(rename = "CharacterName")]
        character_name: String
}

// List allegiance officers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ListAllegianceOfficers {}

// Clear allegiance officers
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_ClearAllegianceOfficers {}

// Recalls to players allegiance hometown
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Allegiance_RecallAllegianceHometown {}

// Admin Returns a plugin list response
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginListResponse {
        #[serde(rename = "Context")]
        context: u32,
        #[serde(rename = "PluginList")]
        plugin_list: String
}

// Admin Returns plugin info
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin_QueryPluginResponse {
        #[serde(rename = "Context")]
        context: u32,
        #[serde(rename = "Success")]
        success: bool,
        #[serde(rename = "PluginName")]
        plugin_name: String,
        #[serde(rename = "PluginAuthor")]
        plugin_author: String,
        #[serde(rename = "PluginEmail")]
        plugin_email: String,
        #[serde(rename = "PluginWebpage")]
        plugin_webpage: String
}

// Completes the barber interaction
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_FinishBarber {
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

// Abandons a contract
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Social_AbandonContract {
        #[serde(rename = "ContractId")]
        contract_id: ContractId
}

// Performs a jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movement_Jump {
        #[serde(rename = "Jump")]
        jump: JumpPack
}

// Move to state data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movement_MoveToState {
        #[serde(rename = "MoveToState")]
        move_to_state: MoveToStatePack
}

// Performs a movement based on input
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movement_DoMovementCommand {
        #[serde(rename = "Motion")]
        motion: u32,
        #[serde(rename = "Speed")]
        speed: f32,
        #[serde(rename = "HoldKey")]
        hold_key: HoldKey
}

// Stops a movement
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movement_StopMovementCommand {
        #[serde(rename = "Motion")]
        motion: u32,
        #[serde(rename = "HoldKey")]
        hold_key: HoldKey
}

// Sets an autonomy level
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movement_AutonomyLevel {
        #[serde(rename = "AutonomyLevel")]
        autonomy_level: u32
}

// Sends an autonomous position
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movement_AutonomousPosition {
        #[serde(rename = "Position")]
        position: AutonomousPositionPack
}

// Performs a non autonomous jump
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Movement_Jump_NonAutonomous {
        #[serde(rename = "Extent")]
        extent: f32
}

// Instructs the client to return to 2D mode - the character list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Login_LogOffCharacter {}

// Mark a character for deletetion.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_CharacterDelete {
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "Slot")]
        slot: i32
}

// Character creation result
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Character_SendCharGenResult {
        #[serde(rename = "Account")]
        account: String,
        #[serde(rename = "Result")]
        result: CharGenResult
}

// The character to log in.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Login_SendEnterWorld {
        #[serde(rename = "CharacterId")]
        character_id: ObjectId,
        #[serde(rename = "Account")]
        account: String
}

// Asks server for a new object description
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Object_SendForceObjdesc {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId
}

// The user has clicked 'Enter'. This message does not contain the Id of the character logging on; that comes later.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Login_SendEnterWorldRequest {}

// Sent if player is an PSR, I assume it displays the server version number
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin_SendAdminGetServerVersion {}

// Seems to be a legacy friends command, /friends old, for when Jan 2006 event changed the friends list
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Social_SendFriendsCommand {
        #[serde(rename = "Command")]
        command: u32,
        #[serde(rename = "Player")]
        player: String
}

// Admin command to restore a character
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Admin_SendAdminRestoreCharacter {
        #[serde(rename = "ObjectId")]
        object_id: ObjectId,
        #[serde(rename = "RestoredCharName")]
        restored_char_name: String,
        #[serde(rename = "AccountToRestoreTo")]
        account_to_restore_to: String
}

// Send or receive a message using Turbine Chat.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "BlobDispatchType")]
pub enum Communication_TurbineChat {
    #[serde(rename = "0x01")]
    #[serde(alias = "0x02")]
    Type01 {
        #[serde(rename = "MmessageSize")]
        mmessage_size: u32,
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

// DDD request for data
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDD_RequestDataMessage {
        #[serde(rename = "ResourceType")]
        resource_type: u32,
        #[serde(rename = "ResourceId")]
        resource_id: DataId
}

// TODO
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDD_InterrogationResponseMessage {
        #[serde(rename = "Language")]
        language: u32,
        #[serde(rename = "Files")]
        files: PackableList
}

// Ends DDD message update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDD_EndDDDMessage {}

// Ends DDD update
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DDD_OnEndDDD {}

