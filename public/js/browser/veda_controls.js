// Veda controls implemented as JQuery plugins
;(function( $ ) { "use strict";

// Generic control behaviour
	$.fn.vedaControl = function( $el, options ) {
		var opts = $.extend( {}, $.fn.vedaControl.defaults, options ),
			control = $(opts.template),
			immutable = opts.immutable;

		$("[bound]", control)
			.val(opts.value)
			.on("change", function ( e ) {
				var value = opts.inputParser( this.value, this );
				if (value !== null) opts.change(value);
				if (!opts.holdValue) this.value = null;
			});	
		
		return control;
	};
	$.fn.vedaControl.defaults = {
		change: function (value) { alert(value) },
		inputParser: function (input) { return input; },
		value: undefined
	};

	// String control
	$.fn.vedaString = function( options ) {
		var opts = $.extend( {}, $.fn.vedaString.defaults, options ),
			control = $.fn.vedaControl(this, opts);

		$("[bound]", control).data("language", opts.value.language);

		$(".language-selector", control).prepend(opts.value.language);

		var first = $("<li>").append( $("<a>", {href: "#", "data-language": "", text: "-"}).addClass("language") );
		if (!opts.value.language) first.addClass("active");
		$(".language-list", control).append(
			first,
			Object.keys(veda.user.availableLanguages).map(function (language_name) {
				var li = $("<li>"), 
					a = $("<a>", {href: "#", "data-language": language_name, text: language_name}).addClass("language");
				li.append(a);
				if (opts.value.language == language_name) li.addClass("active");
				return li;
			})
		);
		
		$(".language", control).on("click", function ( e ) {
			e.preventDefault();
			$(".language-selector", control)
				.empty()
				.append($(this).data("language"), " <span class='caret'></span>");
			$(".language-list li", control).removeClass("active");
			$(this).parent().addClass("active");
			$("textarea", control)
				.data("language", $(this).data("language") )
				.trigger("change");
		});
		
		/*$("textarea", control)
			.autosize()
			.on("focus", function (event) {
				$(this).trigger("autosize.resize");
			});*/

		this.append(control);
		return this;
	};
	$.fn.vedaString.defaults = {
		value: new String(""),
		template: $("#string-control-template").html(),
		change: function (value) { alert(value + " : " + value.language) },
		inputParser: function (input, el) {
			var value = new String(input);
			value.language = $(el).data("language");
			return value != "" ? value : null;
		}
	};
	
	// Boolean control	
	$.fn.vedaBoolean = function( options ) {
		var opts = $.extend( {}, $.fn.vedaBoolean.defaults, options ),
			control = $.fn.vedaControl(this, opts);
		this.append(control);
		return this;
	};
	$.fn.vedaBoolean.defaults = {
		value: new Boolean(false),
		template: $("#boolean-control-template").html(),
		inputParser: function (input) {
			return new Boolean(input == "true" ? true : false);
		}
	};	

	// Integer control
	$.fn.vedaInteger = function( options ) {
		var opts = $.extend( {}, $.fn.vedaInteger.defaults, options ),
			control = $.fn.vedaControl(this, opts);
		this.append(control);
		return this;
	};
	$.fn.vedaInteger.defaults = {
		value: undefined,
		template: $("#integer-control-template").html(),
		inputParser: function (input) {
			var int = parseInt(input, 10);
			return !isNaN(int) ? new Number(int) : null;
		}
	};

	// Decimal control
	$.fn.vedaDecimal = function( options ) {
		var opts = $.extend( {}, $.fn.vedaDecimal.defaults, options ),
			control = $.fn.vedaControl(this, opts);
		this.append(control);
		return this;
	};
	$.fn.vedaDecimal.defaults = {
		value: undefined,
		template: $("#decimal-control-template").html(),
		inputParser: function (input) {
			var float = parseFloat(input);
			return !isNaN(float) ? new Number(float) : null;
		}
	};

	// Datetime control
	$.fn.vedaDatetime = function (options) {
		var opts = $.extend( {}, $.fn.vedaDatetime.defaults, options ),
			control = $.fn.vedaControl(this, opts);
		this.append(control);
		return this;
	};
	$.fn.vedaDatetime.defaults = {
		value: undefined,
		template: $("#datetime-control-template").html(),
		inputParser: function (input) {
			var timestamp = Date.parse(input);
			return !isNaN(timestamp) ? new Date(timestamp) : null;
		}
	};

	// Source code control
	$.fn.vedaSource = function (options) {
		var self = this,
			opts = $.extend( {}, $.fn.vedaSource.defaults, options ),
			immutable = opts.immutable,
			control = $(opts.template),
			fscreen = $("#full-screen", control),
			editorEl = control.get(0),
			editor = CodeMirror(editorEl, {
				value: opts.value.toString(),
				mode: opts.mode,
				matchBrackets: true,
				autoCloseBrackets: true,
				matchTags: true,
				autoCloseTags: true,
				lineNumbers: true
			});
		setTimeout( function () {
			editor.refresh();
		}, 100);
		this.on("view edit search", function (e) {
			e.stopPropagation();
			editor.refresh();
			e.type === "view"   ? ( editor.setOption("readOnly", "nocursor") ) : 
			e.type === "edit" && !immutable ? ( editor.setOption("readOnly", false) ) : 
			e.type === "edit" && immutable ? ( editor.setOption("readOnly", "nocursor") ) : 
			e.type === "search" ? ( editor.setOption("readOnly", false) ) :
			true;
		});
		editor.on("change", function () {
			var value = opts.inputParser( editor.doc.getValue() );
			opts.change(value);
		});
		fscreen.click(function () {
			var body = $("body"),
			    all = $("body > *"),
				parent = control.parent(),
				wrapper = $("<div class='fs-wrapper'></div>"),
				cm = $(".CodeMirror", control);
			if ( !parent.hasClass("fs-wrapper") ) {
				control.wrap( wrapper );
				cm.addClass("CodeMirror-fs");
				parent = control.parent();
				parent.detach();
				all.hide();
				body.append(parent);
			} else {
				parent.detach();
				cm.removeClass("CodeMirror-fs");
				self.append(parent);
				control.unwrap();
				all.show();
			}
			control.toggleClass("fs");
			fscreen.toggleClass("glyphicon-resize-full glyphicon-resize-small");
			editor.refresh();
		});
		this.append(control);
		return this;
	}
	$.fn.vedaSource.defaults = {
		value: new String(""),
		template: $("#source-control-template").html(),
		mode: "javascript", 
		inputParser: function (input) {
			return new String(input);
		}
	};
	
	// Object property control
	$.fn.vedaLink = function( options ) {
		var opts = $.extend( {}, $.fn.vedaLink.defaults, options ),
			control = $(opts.template),
			queryPrefix = opts.queryPrefix,
			add = $(".add", control),
			all = $(".all", control);
		
		if (!opts.add) {
			add.remove();
		} else {
			add.click(opts.add);
		}

		if (!queryPrefix) {
			all.remove();
		} else {
			all.click(function () {
				typeAhead.data().ttTypeahead.input.trigger("queryChanged", "");
				typeAhead.focus();
			});
		}

		var typeAhead = $(".typeahead", control).typeahead (
			{
				minLength: 0,
				highlight: true
			},
			{
				name: "dataset",
				source: function (q, cb) {
					var limit = opts.limit || -1;
					var s = new veda.SearchModel(q, null, queryPrefix);
					var results = [];
					for (var uri in s.results) {
						if (limit-- === 0) break;
						results.push(s.results[uri]);
					}
					cb(results);
				},
				displayKey: function (individual) {
					var result;
					try { result = riot.render("{rdf:type.0.rdfs:label}: {rdfs:label}", individual); }
					catch (ex) { result = individual.id; }
					return result;
				}
			}
		);

		typeAhead.on("typeahead:selected", function (e, selected) {
			opts.select(selected);
		});

		// Search modal
		var tmpl = $("#search-modal-template").html();
		
		$(".search", control).on("click", function (e) {
			var $modal = $(tmpl);
			$("body").append($modal);
			$modal.modal();
			var search = new veda.SearchModel(undefined, $(".modal-body", $modal), queryPrefix);
			// Add found values
			$("button#ok", $modal).on("click", function (e) {
				$(this).off("click");
				var selected = [];
				for (var uri in search.selected) {
					selected.push( search.selected[uri] );
				}
				opts.select(selected);
			});
			$modal.on('hidden.bs.modal', function (e) {
				$modal.remove();
			});
		});
		
		this.append(control);
		return this;
	};
	$.fn.vedaLink.defaults = {
		template: $("#link-control-template").html(),
		queryPrefix: ""
	};


})( jQuery );
