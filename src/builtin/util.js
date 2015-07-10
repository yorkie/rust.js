(function (exports) {
	'use strict'
	
	/**
	 * @method objectToString
	 * @private
	 * @param {object} obj
	 * @return {string}
	 */
	function objectToString (o) {
  	return Object.prototype.toString.call(o);
	}
	
	/**
	 * @method hasOwnProperty
	 * @private
	 * @param {any} arg
	 * @return {boolean}
	 */
	function hasOwnProperty(obj, prop) {
  	return Object.prototype.hasOwnProperty.call(obj, prop);
	}
	
	/**
	 * @method isArray
	 * @param {any} arg
	 * @return {boolean}
	 */
	exports.isArray = Array.isArray;
	
	/**
	 * @method isBoolean
	 * @param {any} arg
	 * @return {boolean}
	 */
	exports.isBoolean = function isBoolean (arg) {
		return typeof arg === 'boolean';
	};
	
	/**
	 * @method isNull
	 * @param {any} arg
	 * @return {boolean}
	 */
	exports.isNull = function isNull (arg) {
		return arg === null;
	};
	
	/**
	 * @method isNullOrUndefined
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isNullOrUndefined (arg) {
  	return arg === null || arg === undefined;
	}
	exports.isNullOrUndefined = isNullOrUndefined;

	/**
	 * @method isNumber
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isNumber (arg) {
  	return typeof arg === 'number';
	}
	exports.isNumber = isNumber;

	/**
	 * @method isString
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isString (arg) {
  	return typeof arg === 'string';
	}
	exports.isString = isString;

	/**
	 * @method isSymbol
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isSymbol (arg) {
  	return typeof arg === 'symbol';
	}
	exports.isSymbol = isSymbol;

	/**
	 * @method isUndefined
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isUndefined (arg) {
  	return arg === undefined;
	}
	exports.isUndefined = isUndefined;

	/**
	 * @method isRegExp
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isRegExp (re) {
  	return re !== null && typeof re === 'object' &&
         objectToString(re) === '[object RegExp]';
	}
	exports.isRegExp = isRegExp;

	/**
	 * @method isObject
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isObject (arg) {
  	return arg !== null && typeof arg === 'object';
	}
	exports.isObject = isObject;

	/**
	 * @method isDate
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isDate (d) {
  	return d !== null && typeof d === 'object' &&
    	     objectToString(d) === '[object Date]';
	}
	exports.isDate = isDate;

	/**
	 * @method isError
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isError (e) {
	  return e !== null && typeof e === 'object' &&
	      (objectToString(e) === '[object Error]' || e instanceof Error);
	}
	exports.isError = isError;
	
	/**
	 * @method isFunction
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isFunction (arg) {
	  return typeof arg === 'function';
	}
	exports.isFunction = isFunction;
	
	/**
	 * @method isPrimitive
	 * @param {any} arg
	 * @return {boolean}
	 */
	function isPrimitive (arg) {
	  return arg === null ||
	         typeof arg !== 'object' && typeof arg !== 'function';
	}
	exports.isPrimitive = isPrimitive;
	
	/**
	 * Inherit the prototype methods from one constructor into another.
	 *
	 * The Function.prototype.inherits from lang.js rewritten as a standalone
	 * function (not on Function.prototype). NOTE: If this file is to be loaded
	 * during bootstrapping this function needs to be rewritten using some native
	 * functions as prototype setup using normal JavaScript does not work as
	 * expected during bootstrapping (see mirror.js in r114903).
	 *
	 * @param {function} ctor Constructor function which needs to inherit the
	 *     prototype.
	 * @param {function} superCtor Constructor function to inherit prototype from.
	 * @throws {TypeError} Will error if either constructor is null, or if
	 *     the super constructor lacks a prototype.
	 */
	exports.inherits = function (ctor, superCtor) {
		if (ctor === undefined || ctor === null)
    	throw new TypeError('The constructor to `inherits` must not be ' +
                        	'null or undefined.');
  	if (superCtor === undefined || superCtor === null)
    	throw new TypeError('The super constructor to `inherits` must not ' +
                        	'be null or undefined.');
  	if (superCtor.prototype === undefined)
    	throw new TypeError('The super constructor to `inherits` must ' +
                        	'have a prototype.');
  	ctor.super_ = superCtor;
  	ctor.prototype = Object.create(superCtor.prototype, {
    	'constructor': {
      	value: ctor,
      	enumerable: false,
      	writable: true,
      	configurable: true
    	}
  	});
	};
	
	
	exports._extend = function(origin, add) {
	  // Don't do anything if add isn't an object
	  if (add === null || typeof add !== 'object') return origin;
	
	  var keys = Object.keys(add);
	  var i = keys.length;
	  while (i--) {
	    origin[keys[i]] = add[keys[i]];
	  }
	  return origin;
	};
	
})