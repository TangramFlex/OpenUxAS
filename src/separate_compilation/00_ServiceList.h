// ===============================================================================
// Authors: AFRL/RQQA
// Organization: Air Force Research Laboratory, Aerospace Systems Directorate, Power and Control Division
// 
// Copyright (c) 2017 Government of the United State of America, as represented by
// the Secretary of the Air Force.  No copyright is claimed in the United States under
// Title 17, U.S. Code.  All Other Rights Reserved.
// ===============================================================================

/* 
 * File:   ServiceList.h
 * Author: steve
 *
 * Created on March 29, 2017, 4:47 PM
 */

/*! \brief This file is used to register services. First the service header is
 * "included" and the top of the service manager, and then the service is registered
 * in ServiceManager::getInstance(), through the creation of a "dummy" instance. 
 * To add new services: 
 * 1) add a #include statement, for the service, in the SERVICE HEADER FILES SECTION. 
 * 2) add a line to create a "dummy" instance in the SERVICE REGISTRATION SECTION. 
 * 3) add a #include statement, for each task, in the INCLUDE TASK MESSAGES SECTION. 
 * 4) add a subscription, for each task, in the SUBSCRIBE TO TASKS SECTION. 
*/


//////////////////////////////////////////////////////////////////////////////////////
//define INCLUDE_SERVICE_HEADERS to include header files at top of service manager ///
//////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////
//define REGISTER_SERVICE_CODE to register the services in the     service manager ///
//////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////
//define INCLUDE_TASK_MESSAGES to to include headers for all, current task messages///
//////////////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////////////
//define SUBSCRIBE_TO_TASKS to subscribe to all tasks                              ///
//////////////////////////////////////////////////////////////////////////////////////

#include "config.h"

#ifdef AFRL_INTERNAL_ENABLED
#include "AFRLInternalServices.h"
#endif

//////////////////////////////////////////////////////
/// BEGIN -- SERVICE HEADER FILES SECTION          ///
/// include service header files in this section   ///
//////////////////////////////////////////////////////

#if defined INCLUDE_SERVICE_HEADERS
#undef INCLUDE_SERVICE_HEADERS

#ifndef UXAS_SERVICE_LIST_CODE_HEADERS  // only allow one-time definition
#define UXAS_SERVICE_LIST_CODE_HEADERS


// examples
#include "01_HelloWorld.h"

// data

// task

// DO NOT REMOVE - USED TO AUTOMATICALLY ADD NEW TASK HEADERS


// test

// general services

// DO NOT REMOVE - USED TO AUTOMATICALLY ADD NEW SERVICE HEADERS


#endif  //UXAS_SERVICE_LIST_CODE_HEADERS
#endif  //INCLUDE_SERVICE_HEADERS

//////////////////////////////////////////////////////
/// END -- SERVICE HEADER FILES SECTION            ///
//////////////////////////////////////////////////////




//////////////////////////////////////////////////////////
/// BEGIN -- SERVICE REGISTRATION SECTION              ///
/// create dummy instances of services in this section ///
//////////////////////////////////////////////////////////

#if defined REGISTER_SERVICE_CODE   // define this to register the services
#undef REGISTER_SERVICE_CODE



// examples

// data

// task

// DO NOT REMOVE - USED TO AUTOMATICALLY ADD NEW TASK DUMMY INSTANCES


// test

// general services

// DO NOT REMOVE - USED TO AUTOMATICALLY ADD NEW SERVICE DUMMY INSTANCES


#endif  //REGISTER_SERVICE_CODE
//////////////////////////////////////////////////////////
/// END -- SERVICE REGISTRATION SECTION                ///
//////////////////////////////////////////////////////////
