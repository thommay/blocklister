# Blackholer!

Generate PowerDNS compatible lua files from blocklists.
It's fairly trivial to achieve PiHole functionality with pdns_recursor.

First, select your blocklists and add them to `config.toml`.

Run the blackholer, with `cargo run -- config.toml`. By default, you'll get a file called `blocklist.lua`.

Add `lua-dns-script=/path/to/adblock.lua` to your `recursor.conf`.

Place the following in adblock.lua:
```lua
adservers=newDS()

function preresolve(dq)
	if(not adservers:check(dq.qname)) then
		return false
	end
	
	if(dq.qtype == pdns.A) then
		dq:addAnswer(dq.qtype, "127.0.0.1")
	elseif(dq.qtype == pdns.AAAA) then
		dq:addAnswer(dq.qtype, "::1")
	end
	return true
end

adservers:add(dofile("/path/to/blocklist.lua"))
```
and start your Recursor.

Lua from https://gist.github.com/ahupowerdns/bb1a043ce453a9f9eeed originally.

