// Veda HTTP server functions

$.ajaxSetup ({
	dataType: "json",
	cache: false
});

function get_rights(ticket, uri, callback) {
	var params = {
		type: "GET",
		url: "get_rights",
		data: { "ticket": ticket, "uri": uri }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function get_rights_origin(ticket, uri, callback) {
	var params = {
		type: "GET",
		url: "get_rights_origin",
		data: { "ticket": ticket, "uri": uri }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function authenticate(login, password, callback) {
	var params = {
			type: "GET",
			url: "authenticate",
			data: { "login": login, "password": password }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function is_ticket_valid(ticket, callback) {
	var params = {
		type: "GET",
		url: "is_ticket_valid",
		data: { "ticket": ticket }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return result.responseText;
	}
}

function wait_pmodule(pmodule_id, callback) {
	var params = {
		type: "GET",
		url: "wait_pmodule",
		data: { "pmodule_id": pmodule_id }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function backup(callback) {
	var params = {
		type: "GET",
		url: "backup",
		data: { }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function count_individuals(callback) {
	var params = {
		type: "GET",
		url: "count_individuals",
		data: { }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function set_trace(idx, state, callback) {
	var params = {
		type: "GET",
		url: "set_trace",
		data: { "idx": idx, "state" : state  }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function query(ticket, q, sort, databases, reopen, callback) {
	var params = {
		type: "GET",
		url: "query",
		data: { "ticket": ticket, "query": q, "sort": sort || null, "databases" : databases || null, "reopen" : reopen || false }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function get_individuals(ticket, uris, callback) {
	var params = {
		type: "POST",
		url: "get_individuals",
		data: JSON.stringify({ "ticket": ticket, "uris": uris }),
		contentType: "application/json"
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

var get_count = 0, get_summary_time = 0;

function get_individual(ticket, uri, callback) {
	
	var t1, t2;
	t1 = Date.now();
	get_count++;
	
	var params = {
		type: "GET",
		url: "get_individual",
		data: { "ticket": ticket, "uri": uri }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		
		t2 = Date.now();
		get_summary_time += t2 - t1;
		
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText, function dateReviver (key, value) {
			return key === "data" && this.type === "Datetime" ? new Date(value) : value;
		});
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function put_individual(ticket, individual, wait_for_indexing, callback) {
	var params = {
		type: "PUT",
		url: "put_individual",
		data: JSON.stringify({"ticket": ticket, "individual": individual, "wait_for_indexing" : wait_for_indexing || false }),
		contentType: "application/json"
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function add_to_individual(ticket, individual, wait_for_indexing, callback) {
	var params = {
		type: "PUT",
		url: "add_to_individual",
		data: JSON.stringify({"ticket": ticket, "individual": individual, "wait_for_indexing" : wait_for_indexing || false }),
		contentType: "application/json"
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function set_in_individual(ticket, individual, wait_for_indexing, callback) {
	var params = {
		type: "PUT",
		url: "set_in_individual",
		data: JSON.stringify({"ticket": ticket, "individual": individual, "wait_for_indexing" : wait_for_indexing || false }),
		contentType: "application/json"
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function remove_from_individual(ticket, individual, wait_for_indexing, callback) {
	var params = {
		type: "PUT",
		url: "remove_from_individual",
		data: JSON.stringify({"ticket": ticket, "individual": individual, "wait_for_indexing" : wait_for_indexing || false }),
		contentType: "application/json"
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}





function get_property_value(ticket, uri, property_uri, callback) {
	var params = {
		type: "GET",
		url: "get_property_value",
		data: { "ticket": ticket, "uri": uri, "property_uri": property_uri }
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}

function execute_script(script, callback) {
	var params = {
		type: "POST",
		url: "execute_script",
		data: JSON.stringify({"script": script}),
		contentType: "application/json"
	};
	if(!callback) {
		params.async = false;
		var result = $.ajax(params);
		if (result.status >= 400) throw {status: result.status, description: result.statusText};
		return JSON.parse(result.responseText);
	}
	$.ajax(params)
		.fail( function () { throw {status: result.status, description: result.statusText}; } )
		.done( function (data) { callback(data); } );
}
