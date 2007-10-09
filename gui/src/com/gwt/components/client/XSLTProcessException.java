/**
 * Created on Sep 4, 2006
 */
package com.gwt.components.client.xslt;

/**
 * An exception for XSLT processing errors
 * @author Eric Bessette <ebessette@gmail.com>
 */
public class XSLTProcessException extends Exception {

	/**
	 * The serial version uid
	 */
	private static final long	serialVersionUID	= 5435735969790787603L;

	/**
	 * Creates a new XSLT Process Exception
	 */
	public XSLTProcessException() {

		super();
	}

	/**
	 * Creates a new XSLT Process Exception
	 * @param msg The message for this exception
	 */
	public XSLTProcessException( String msg ) {

		super( msg );
	}

	/**
	 * Create a new XSLT Process Exception
	 * @param msg The message for this exception
	 * @return A new parse exception
	 */
	public static XSLTProcessException create( String msg ) {

		return new XSLTProcessException( msg );
	}

	/**
	 * Creates a new XSLT Process Exception
	 * @param msg The message for this exception
	 * @param cause The child cause for this exception
	 */
	public XSLTProcessException( String msg, Throwable cause ) {

		super( msg, cause );
	}

	/**
	 * Creates a new XSLT Process Exception
	 * @param cause The child cause for this exception
	 */
	public XSLTProcessException( Throwable cause ) {

		super( cause );
	}
}
