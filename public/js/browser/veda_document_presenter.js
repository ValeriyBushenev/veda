// Document Presenter

veda(function DocumentPresenter(app) { "use strict";
	app.on("load:document", function () {
	// Get or create Model
	var doc = app.document || RegisterModule(new app.DocumentModel(), app, "document");

	// Render View
	var template = $("#document-template").html();
	$("#main").html(template);

	// Listen View changes & update Model
	$("#document #load").on("click", function(event) {
		event.preventDefault();
	});
	$("#document #save").on("click", function(event) {
		event.preventDefault();
	});

	// Listen Model changes & update View
	doc.on("loaded", function() {
	});
	doc.on("saved", function() {
	});
	});
});
