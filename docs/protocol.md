# Protocol

Nadir uses a simple JSON websocket protocol to transport notify messages. This will be called "Nadir Notify Protocol" (NNP) in the following documents.

NNP contains two participants: _the client_ usually refers to `nadir-notify` program or compatible implementations; _the server_ refers to any other program that sends notify messages to the client.

## Data Model

Every notify message is a `Message` in NNP. Each `Message` belongs to exactly one `MessageGroup`.

A `MessageGroup` is the place you're displaying your message in. It has two message slots: pinned and not-pinned. Pinned messages will show before not-pinned ones. In either slots, messages are sorted in newest-first order. It also has a message counter field to indicate how many messages are in this group.

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

All message sent and received by the Client MUST be a JSON string. Every message MUST contain a string field named `_t` to indicate the type of the message.

```ts
interface ApiMessage {
    _t: string
}
```

### Server Messages

Most messages in NNP are sent from the Server.

```ts
interface ServerMessage extends ApiMessage {
    _t: 'put_group' | 'remove_group' | 'put' | 'remove'
}
```

`put_group` and `remove_group` updates `MessageGroup`s in the client. `put_group` will update group information for an existing group or add a new group, and `remove_group` will remove the given group if it exists.

```ts
interface PutGroupMessage extends ServerMessage {
    _t: 'put_group'
    group: MessageGroup
}

interface RemoveGroupMessage extends ServerMessage {
    _t: 'remove_group'
    group: string
}
```

`put` and `remove` updates messages in a certain `MessageGroup`.

TODO

### Client Messages

TODO

## Connection

The connection can be initiated from either the Client or the Server.

TODO: verification?
