# Protocol

Nadir uses a simple JSON websocket protocol to transport notify messages.

The protocol contains two participants: 

- _The Frontend_ usually refers to `nadir-notify` program or other compatible implementations. They display notify messages to the user, and may optionally receive user interaction.
- _The Backend_ refers to other programs that send notify messages to the Frontend.

## Data Model

Every notify message is a `Message` in NNP. Each `Message` belongs to exactly one `MessageGroup`.

A `MessageGroup` is the place you're displaying your message in. It has two message slots: _pinned_ and _not-pinned_. Pinned messages will show before not-pinned ones. In both slots, messages are sorted in newest-first order. It also has a message counter field to indicate how many messages are in this group.

A `Message` has an "unique" `id` to identify it within its group and slot. When adding two message with the same `id` into the same slot, the one added later will replace the earlier one.

```ts
interface Message {
    /** A unique identifier for this message.
    */
    id: string

    /** A number attached to this message, showing how many "real" notifications
        are covered by this message. Setting it to 0, 1 or undefined (omitted) 
        will disable this counter.
    */
    counter: uint64 | undefined

    /** The text section of this message. Usually the title.
    */
    body: string

    /** Strings you'd like to show beside the `body` field. Usually information
        about the sender of the underlying message.
    */
    tags: string[] | undefined

    /** The send time of this message. Should be serialized as a string in
        ISO-8601 format.
    */
    time: DateTime | undefined
}

interface MessageGroup {
    /** The unique identifier of this message group.
    */
    id: string

    /** The title of this group. Usually the source of messages.
    */
    title: string

    /** An integer indicating how import this group is. Groups with higher 
        importance will show in the front. Ordering of groups with the same
        importance is unspecified.
    */
    importance: int32

    /** A message capacity hint for this group.

        This MessageGroup will display at most this many messages, although 
        implementations are free to choose a smaller number when applicable.

        This field governs the not-pinned message slot.
    */
    capacity: uint32

    /** Similar to `capacity`, but governs the pinned message slot.
    */
    pinned_capacity: uint32
}
```

## Messages

All message sent and received by the Frontend must be a JSON string. Every message must contain a string field named `_t` to indicate the type of the message.

```ts
interface ApiMessage {
    _t: string
}
```

### Backend Messages

TODO: Add `Hello` message for exchanging metadata & verifying backends?

Most messages in NNP are sent from the Backend.

```ts
interface BackendMessage extends ApiMessage {
    _t: 'put_group' | 'remove_group' | 'put' | 'remove' |
        'set_group_counter' | 'req_snapshot'
}
```

`put_group` and `remove_group` updates `MessageGroup`s in the Frontend. `put_group` will update group information for an existing group or add a new group. `remove_group` will remove the given group, and any messages in this group, if it exists.

```ts
interface PutGroupMessage extends BackendMessage {
    _t: 'put_group'
    group: MessageGroup
}

interface RemoveGroupMessage extends BackendMessage {
    _t: 'remove_group'
    group: string
}
```

`put` and `remove` updates messages in a certain `MessageGroup`. They can add or remove multiple messages in a group at once.

```ts
interface PutMessage extends BackendMessage {
    _t: 'put'
    group: string
    items: Message[]
}

interface RemoveMessage extends BackendMessage {
    _t: 'remove'
    items: string[]
}
```

`set_group_counter` sets the counter in the specified group. This message is separated from other group messages because it's called more frequently.

```ts
interface SetGroupCounterMessage extends BackendMessage {
    _t: 'set_group_counter'
    group: string
    counter: uint64
}
```

The backend can also request a snapshot of a certain group via `req_snapshot`. The backend must supply a unique `msg_id` for the frontend to response.

```ts
interface MessageRequiringResponse extends BackendMessage {
    _t: 'req_snapshot'
    msg_id: string
}

interface RequestSnapshotMessage extends MessageRequiringResponse {
    _t: 'req_snapshot'
    group: string
}
```

TODO

### Frontend Messages

The Frontend may send messages in response of user action.

```ts
interface FrontendMessage {
    _t: 'user_action' | 'resp_snapshot'
}
```

A `user_action` message indicates that the user has performed some kind of action on a specific notify message. This feature has not yet been implemented, so the Frontend might send this kind of message to any client.

The only available action at this time is `click`, which represents one clicking on or pressing Space / Enter when selecting this message.

```ts
interface UserActionMessage extends FrontendMessage {
    _t: 'user_action'
    group: string
    message: string
    action: 'click'
}
```

The Frontend may also send messages in reply to some requests. These response may not be in the same order as the requests, and may be separated by non-response messages.

A `resp_snapshot` message is the response of the `req_snapshot` message. It contains the definition of the group and messages currently stored in frontend.

```ts
interface ResponseMessage extends BackendMessage {
    _t: 'resp_snapshot'
    reply_to: string
}

interface SnapshotResponseMessage extends ResponseMessage {
    _t: 'resp_snapshot'
    group: MessageGroup
    messages: Message[]
}
```

TODO

## Connection

The connection can be initiated from either the Frontend or the Backend, with a WebSocket connection request to the other side. The connection COULD be a plain connection or a secure (wss) one, but the latter is preferred.

TODO: verification? ->

A shared secret COULD be used to authorize clients. If such secret is set, the frontend MUST send a nonce string in `FrontendHelloMessage` (TODO). In response, the backends MUST send `hex(hmac_sha256(nonce, secret))` in its `BackendHelloMessage`(TODO). If the value doesn't match, the connection SHOULD be dropped immediately.
